use nix::libc::{SIGCHLD, SIGINT, SIGTSTP};
use rustyline::{error::ReadlineError, Editor};
use signal_hook::iterator::Signals;
use std::{
    error::Error,
    sync::mpsc::{channel, sync_channel, Sender},
    thread,
};

pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

enum WorkerMsg {
    Signal(i32), // シグナル入力
    Cmd(String), // コマンド入力
}

fn main() -> Result<(), DynError> {
    let mut rl = Editor::<()>::new()?;

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.")
    }

    let (worker_tx, worker_rx) = channel();

    // SIGINIT, SIGTSTP, SIGCHILDがきたら、worker_rxに飛ばすスレッドを起動
    spawn_sig_handler(worker_tx.clone())?;

    // TODO recipeverの処理だけ書く(worker::newのspawnはリッチだからまず受け取るだけのものを書く)

    let line = "aaa".to_string();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                worker_tx.send(WorkerMsg::Cmd(line)).unwrap();
                // rl.add_history_entry(line.as_str());
                // println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt")?;
    Ok(())
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
