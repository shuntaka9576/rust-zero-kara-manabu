use crate::helper::DynError;
use nix::{
    libc,
    sys::{
        signal::{killpg, signal, SigHandler, Signal},
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{self, dup2, execv, fork, pipe, setpgid, tcgetpgrp, tcsetpgrp, ForkResult, Pid},
};
use rustyline::{error::ReadlineError, Editor};
use signal_hook::{consts::*, iterator::Signals};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::CString,
    fmt::format,
    mem::replace,
    path::PathBuf,
    process::exit,
    str::pattern::Pattern,
    sync::mpsc::{channel, sync_channel, Receiver, Sender, SyncSender},
    thread,
};

fn syscall<F, T>(f: F) -> Result<T, nix::Error>
where
    F: Fn() -> Result<T, nix::Error>,
{
    loop {
        match f() {
            Err(nix::Error::EINTR) => (), // リトライ
            result => return result,
        }
    }
}

enum WorkerMsg {
    Signal(i32), // シグナルを受信
    Cmd(String), // コマンド入力
}

// mainスレッドが受信するメッセージ
enum ShellMsg {
    Continue(i32), // シェルの読み込みを再開。i32は最後の終了コード
    Quit(i32),     // シェルを終了。i32はシェルの終了コード
}

#[derive(Debug)]
pub struct Shell {
    logfile: String,
}

impl Shell {
    pub fn new(logfile: &str) -> Self {
        Shell {
            logfile: logfile.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), DynError> {
        unsafe { signal(Signal::SIGTTOU, SigHandler::SigIgn).unwrap() };

        let mut rl = Editor::<()>::new()?;
        if let Err(e) = rl.load_history(&self.logfile) {
            eprintln!("ZeroSh: ヒストリファイルの読み込みに失敗: {e}");
        }

        let (worker_tx, worker_rx) = channel();
        let (shell_tx, shell_rx) = sync_channel(0);
        spawn_sig_handler(worker_tx.clone())?;
        Worker::new().spawn(worker_rx, shell_tx);

        let exit_val;
        let mut prev = 0;
        loop {
            let face = if prev == 0 { '\u{1F642}' } else { '\u{1F480}' };
            match rl.readline(&format!("ZeroSh {face} %> ")) {
                Ok(line) => {
                    let line_trimed = line.trim(); // 前後の空白文字を削除
                    if line_trimed.is_empty() {
                        continue; // 空のコマンドの場合は再読み込み
                    } else {
                        rl.add_history_entry(line_trimed); // ヒストリファイルに追加
                    }

                    worker_tx.send(WorkerMsg::Cmd(line)).unwrap(); // workerに送信
                    match shell_rx.recv().unwrap() {
                        ShellMsg::Continue(n) => prev = n, // 読み込み再開
                        ShellMsg::Quit(n) => {
                            // シェルを終了
                            exit_val = n;
                            break;
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => eprintln!("ZeroSh: 終了はCtrl+d"),
                Err(ReadlineError::Eof) => {
                    worker_tx.send(WorkerMsg::Cmd("exit".to_string())).unwrap();
                    match shell_rx.recv().unwrap() {
                        ShellMsg::Quit(n) => {
                            exit_val = n;
                            break;
                        }
                        _ => panic!("exitに失敗"),
                    }
                }
                Err(e) => {
                    eprintln!("ZeroSh: 読み込みエラー\n{e}");
                    exit_val = 1;
                    break;
                }
            }
        }

        if let Err(e) = rl.save_history(&self.logfile) {
            eprintln!("ZeroSh: ヒストリファイルへの書き込みに失敗: {e}");
        }
        exit(exit_val);
    }
}

fn spawn_sig_handler(tx: Sender<WorkerMsg>) -> Result<(), DynError> {
    let mut signals = Signals::new(&[SIGINT, SIGTSTP, SIGCHLD])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            tx.send(WorkerMsg::Signal(sig)).unwrap()
        }
    });

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ProcState {
    Run,
    Stop,
}

#[derive(Debug, Clone)]
struct ProcInfo {
    state: ProcState,
    pgid: Pid,
}

#[derive(Debug)]
struct Worker {
    exit_val: i32,   // 終了コード
    fg: Option<Pid>, // フォアグラウンドのプロセスグループID

    // ジョブIDから(プロセスID、実行コマンド)へのマップ
    jobs: BTreeMap<usize, (Pid, String)>,

    pgid_to_pids: HashMap<Pid, (usize, HashSet<Pid>)>, // プロセスグループIDから(ジョブID、プロセスID)へのマップ
    pid_to_info: HashMap<Pid, ProcInfo>,
    shell_pgid: Pid, // シェルのプロセスグループID
}

impl Worker {
    fn new() -> Self {
        Worker {
            exit_val: 0,
            fg: None,
            jobs: BTreeMap::new(),
            pgid_to_pids: HashMap::new(),
            pid_to_info: HashMap::new(),
            shell_pgid: tcgetpgrp(libc::STDIN_FILENO).unwrap(),
        }
    }

    fn built_in_cmd(&mut self, cmd: &[(&str, Vec<&str>)], shell_tx: &SyncSender<ShellMsg>) -> bool {
        if cmd.len() > 1 {
            return false;
        }

        match cmd[0].0 {
            "exit" => self.run_exit(&cmd[0].1, shell_tx),
            "jobs" => self.run_jobs(shell_tx),
            "fg" => self.run_fg(&cmd[0].1, shell_tx),
            "cd" => self.run_cd(&cmd[0].1, shell_tx),
            _ => false,
        }
    }

    fn run_cd(&mut self, args: &[&str], shell_tx: &SyncSender<ShellMsg>) -> bool {
        let path = if args.len() == 1 {
            dirs::home_dir()
                .or_else(|| Some(PathBuf::from("/")))
                .unwrap()
        } else {
            PathBuf::from(args[1])
        };

        if let Err(e) = std::env::set_current_dir(&path) {
            self.exit_val = 1;
            eprintln!("cdに失敗: {e}");
        } else {
            self.exit_val = 0;
        }

        shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
        true
    }

    fn run_exit(&mut self, args: &[&str], shell_tx: &SyncSender<ShellMsg>) -> bool {
        if !self.jobs.is_empty() {
            eprintln!("ジョブが実行中なので終了できません");
            self.exit_val = 1;
            shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
            return true;
        }

        let exit_val = if let Some(s) = args.get(1) {
            if let Ok(n) = (*s).parse::<i32>() {
                n
            } else {
                eprintln!("{s}は不正な引数です");
                self.exit_val = 1;
                shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
                return true;
            }
        } else {
            self.exit_val
        };

        shell_tx.send(ShellMsg::Quit(exit_val)).unwrap();

        true
    }

    fn run_jobs(&mut self, shell_tx: &SyncSender<ShellMsg>) -> bool {
        for (job_id, (pgid, cmd)) in &self.jobs {
            let state = if self.is_group_stop(*pgid).unwrap() {
                "停止中"
            } else {
                "実行中"
            };
            println!("[{job_id}] {state}\t{cmd}")
        }
        self.exit_val = 0;
        shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
        true
    }

    fn run_fg(&mut self, args: &[&str], shell_tx: &SyncSender<ShellMsg>) -> bool {
        self.exit_val = 1;

        if args.len() < 2 {
            eprintln!("usage: fg 数字");
            shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
            return true;
        }

        if let Ok(n) = args[1].parse::<usize>() {
            if let Some((pgid, cmd)) = self.jobs.get(&n) {
                eprintln!("[{n}]　再開\t{cmd}");

                self.fg = Some(*pgid);
                tcsetpgrp(libc::STDIN_FILENO, *pgid).unwrap();

                killpg(*pgid, Signal::SIGCONT).unwrap();
                return true;
            }
        }

        eprintln!("{}というジョブは見つかりませんでした", args[1]);
        shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
        true
    }

    fn is_group_stop(&self, pgid: Pid) -> Option<bool> {
        for pid in self.pgid_to_pids.get(&pgid)?.1.iter() {
            if self.pid_to_info.get(pid).unwrap().state == ProcState::Run {
                return Some(false);
            }
        }
        Some(true)
    }

    fn spawn(mut self, worker_rx: Receiver<WorkerMsg>, shell_tx: SyncSender<ShellMsg>) {
        thread::spawn(move || {
            for msg in worker_rx.iter() {
                match msg {
                    WorkerMsg::Cmd(line) => {
                        match parse_cmd(&line) {
                            Ok(cmd) => {
                                if self.built_in_cmd(&cmd, &shell_tx) {
                                    continue;
                                }
                            }
                            Err(e) => {
                                // TODO
                            }
                        }
                    }
                    WorkerMsg::Signal(SIGCHILD) => {
                        self.wait_child(&shell_tx);
                    }
                    _ => (),
                }
            }
            // for msg in worker_rx.iter() {
            //     match msg {
            //         WorkerMsg::Cmd(line) => match parse_cmd(&line) {
            //             match parse_cmd(&line) {

            //             Ok(cmd) => {
            //                 if self.built_in_cmd(&cmd, &shell_tx) {
            //                     continue;
            //                 }

            //                 if !self.spawn_child(&line, &cmd) {
            //                     shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap()
            //                 }
            //             }
            //             Err(e) => {
            //                 eprintln!("ZeroSh: {e}");
            //                 shell_tx.send(ShellMsg::Continue(self.exit_val)).unwrap();
            //             }
            //         },
            //         WorkerMsg::Signal(SIGCHILD) => {
            //             self.wait_child(&shell_tx);
            //         }
            //         _ => (), // 無視
            //     }
            // }
        });
    }
}

// type CmdResult<'a> = Result<Vec<(&'a str, Vec<&'a str>)>, DynError>;
//
// fn parse_cmd_one(line: &str) -> Result<(&str, Vec<&str>), DynError> {
//     let cmd: Vec<&str> = line.split(' ').collect();
//     let mut filename = "";
//     let mut args = Vec::new();
//     for (n, s) in cmd.iter().filter(|s| !s.is_empty()).enumerate() {
//         if n == 0 {
//             filename = *s;
//         }
//         args.push(*s)
//     }
//
//     if filename.is_empty() {
//         Err("空のコマンド".into())
//     } else {
//         Ok((filename, args))
//     }
// }
//
// fn parse_pipe(line: &str) -> Vec<&str> {
//     let cmds: Vec<&str> = line.split('|').collect();
//     cmds
// }

// fn parse_cmd(line: &str) -> CmdResult {
//     let cmds = parse_pipe(line);
//     if cmds.is_empty() {
//         return Err("空のコマンド".into());
//     }
//
//     let mut result = Vec::new();
//     for cmd in cmds {
//         let (filename, args) = parse_cmd_one(cmd)?;
//         result.push((filename, args))
//     }
//
//     Ok(result)
// }
