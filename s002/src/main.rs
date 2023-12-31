use matches::assert_matches;
use std::collections::{BTreeSet, LinkedList};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

// 構造体の宣言
struct PCSpec {
    cpus: u16,
    memory: u32,
    storage: Storage,
}

fn main() {
    println!("--- 2.1.1 ---");
    i2_1_1();
    println!("---");
    println!("--- 2.1.2 ---");
    i2_1_2();
    println!("---");
    println!("--- 2.1.3 ---");
    i2_1_3();
    println!("---");
    println!("--- 2.1.5 ---");
    i2_1_5();
    println!("---");
    println!("--- 2.1.6 ---");
    i2_1_6();
    println!("---");
    println!("--- 2.1.7 ---");
    i2_1_7();
    println!("---");
    println!("--- 2.1.8 ---");
    i2_1_8();
    println!("---");
    println!("--- 2.1.9 ---");
    i2_1_9();
    println!("---");
    println!("--- 2.1.10 ---");
    i2_1_10();
    println!("---");
    println!("--- 2.1.11 ---");
    i2_1_11();
    println!("---");
    println!("--- 2.1.11 ---");
    i2_1_12();
    println!("---");
    println!("--- 2.2.1 ---");
    i2_2_1();
    println!("---");
    println!("--- 2.2.2 ---");
    i2_2_2();
    println!("---");
    println!("--- 2.2.2 ---");
    i2_2_3();
    println!("---");
    println!("--- 2.2.4 ---");
    i2_2_4();
    println!("---");
    println!("--- 2.2.5 ---");
    i2_2_5();
    println!("---");
    println!("--- 2.2.6 ---");
    i2_2_6();
    println!("---");
    println!("--- 2.2.7 ---");
    i2_2_7();
    println!("---");
    println!("--- 2.2.8 ---");
    i2_2_8();
    println!("---");
    println!("--- 2.2.9 ---");
    i2_2_9();
    println!("---");
    println!("--- 2.2.10 ---");
    i2_2_10();
    println!("---");
    println!("--- 2.2.11 ---");
    i2_2_11();
    println!("---");
    // 2.2.12は命名規則なのでなし
    println!("--- 2.2.13 ---");
    i2_2_13();
    println!("---");
    println!("--- 2.2.14 ---");
    i2_2_14();
    println!("---");
    println!("--- 2.2.15 ---");
    i2_2_15();
    println!("---");
}

fn i2_2_15() {
    // マルチスレッド
    {
        // spawnとjoin
        fn worker() -> u32 {
            println!("start worker");
            // thread::sleep(Duration::from_millis(1000));
            println!("fin worker");
            100
        }

        let handler = std::thread::spawn(worker);

        // joinで待ち受け
        match handler.join() {
            Ok(n) => println!("{n}"),
            Err(e) => println!("{:?}", e),
        }
    }

    // チャンネル
    {
        println!("  チャンネル");
        let (tx, rx) = std::sync::mpsc::sync_channel(64);

        let handler = std::thread::spawn(move || match rx.recv() {
            Ok((x, y)) => println!("({}, {})", x, y),
            Err(e) => eprintln!("{e}"),
        });

        if let Err(e) = tx.send((10, 20)) {
            eprintln!("{e}");
        }

        if let Err(e) = handler.join() {
            eprintln!("{:?}", e);
        }
    }

    // スレッドを用いた2つのVec値を並列にソートする
    {
        // xorshiftに疑似乱数生成
        struct XOR64 {
            x: u64,
        }

        impl XOR64 {
            fn new(seed: u64) -> XOR64 {
                XOR64 {
                    x: seed ^ 88172645463325252,
                }
            }

            fn next(&mut self) -> u64 {
                let x = self.x;
                let x = x ^ (x << 13);
                let x = x ^ (x >> 7);
                let x = x ^ (x >> 17);
                self.x = x;

                return x;
            }
        }

        const NUM: usize = 200000000;
        fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
            let mut v1 = Vec::new();
            let mut v2 = Vec::new();

            let mut generator = XOR64::new(1234);

            for _ in 0..NUM {
                let x1 = generator.next();
                let x2 = generator.next();
                v1.push(x1);
                v2.push(x2);
            }

            (v1, v2)
        }

        fn single_threaded() {
            println!("start!");
            let (mut v1, mut v2) = randomized_vec();

            let start = std::time::Instant::now(); // 現在時刻

            v1.sort();
            v2.sort();

            let end = start.elapsed();

            println!(
                "single_threaded: {}.{:03}秒",
                end.as_secs(),
                end.subsec_nanos()
            );
            println!("end")
        }

        fn multi_threaded() {
            let (mut v1, mut v2) = randomized_vec();
            let start = std::time::Instant::now();

            let handler1 = std::thread::spawn(move || {
                v1.sort();
                v1
            });

            let handler2 = std::thread::spawn(move || {
                v2.sort();
                v2
            });
            let _v1 = handler1.join().unwrap();
            let _v2 = handler2.join().unwrap();
            let end = start.elapsed();

            println!(
                "multi_threaded: {}.{:03}秒",
                end.as_secs(),
                end.subsec_nanos()
            );
        }

        single_threaded();
        multi_threaded();
    }
}

fn i2_2_14() {
    // イテレーター
    {
        let mut s = BTreeSet::new();
        s.insert(100);
        s.insert(400);
        s.insert(6);
        s.insert(1);

        for n in s.iter() {
            println!("{n}")
        }
    }

    // イテレーターメソッドの使い方
    {
        let mut v = Vec::new();
        v.push(10);
        v.push(5);

        let mut s = BTreeSet::new();
        s.insert(100);
        s.insert(400);

        // Vec型とBtreeSet型をイテレーターに変換して連結
        let it = v.iter().chain(s.iter());

        // 各要素を2倍にして出力
        // cloneの意味(?)
        for n in it.clone().map(|n| n * 2) {
            println!("n: {n}")
        }

        // 各要素の合計を計算
        let total = it.clone().fold(0, |acc, x| acc + x);

        // mapでprintlnする方法

        // filterで偶数のみを取り出し、collectでLinkedListに変換
        let list: LinkedList<_> = it.filter(|n| *n % 2 == 0).collect();

        for a in list.iter() {
            // 10, 100, 400の偶数だけ出力
            println!("iter {a}");
        }

        // vの要素とsの要素をとタプルで結合
        for (n, m) in v.iter().zip(s.iter()) {
            println!("({n}, {m})")
        }
    }
}

fn i2_2_13() {
    // コレクション

    // -- LinkedList ---
    {
        let mut list1 = LinkedList::new();
        list1.push_back(0); // [0]
        list1.push_back(1);
        list1.push_back(2);

        let mut list2 = LinkedList::new();
        list2.push_back(100);
        list2.push_back(200);
        list2.push_back(300);

        list1.append(&mut list2); // list1 = [0, 1, 2, 100, 200, 300]
                                  // list2 == []
        list1.push_front(-10); // list1 = [-10,0, 1, 2, 100, 200, 300]
        for n in &list1 {
            // &にしないと後続のlist1でコンパイルエラーになる(所有権周り?)
            println!("linked list for {n}")
        }

        for n in list1.iter() {
            println!("linked list iter: {n}")
        }
    }

    // -- BTreeMap --
    {
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        m.insert(1, "apple");
        m.insert(1, "orange");
        m.insert(3, "banana");

        if let Some(old) = m.remove(&2) {
            // 2に対応する値を削除
            println!("old: {old}");
        }

        if let Some(value) = m.get(&3) {
            // 3に対応する値への不変参照を取得
            println!("{value}");
        }
    }

    // --- set ---
}

fn i2_2_11() {
    // unsafe {signal(Signal::SIGTOU, SigHandler:SigIgn).unwarp()}
}

fn i2_2_10() {
    // compile-time constant
    const PI: f64 = 3.141592;

    // static variable
    static A: u32 = 100;
    static mut B: u32 = 200;
}

fn i2_2_9() {
    {
        fn get_size(s: &Storage) -> u32 {
            match s {
                Storage::HDD { size: s, .. } => *s,
                Storage::SSD(s) => *s,
            }
        };

        impl Storage {
            // メソッド
            fn get_size(&self) -> u32 {
                match self {
                    Storage::HDD { size: s, .. } => *s,
                    Storage::SSD(s) => *s,
                }
            }

            fn set_size(&mut self, size: u32) {
                match self {
                    Storage::HDD { size: s, .. } => *s = size,
                    Storage::SSD(s) => *s = size,
                }
            }
        }

        let mut s = Storage::SSD(512);
        println!("before: {:?}", s);
        let size = s.get_size();
        s.set_size(1024);
        println!("size: {},after: {:?}", size, s);
    }

    {
        impl PCSpec {
            // 型関連関数(associated function)、第一引数がselfではない
            fn new(cpus: u16, memory: u32, storage: Storage) -> PCSpec {
                PCSpec {
                    cpus,
                    memory,
                    storage,
                }
            }
        }

        let s = Storage::SSD(512);
        let spec = PCSpec::new(8, 32, s);
    }

    {
        impl Storage {
            const MAX: usize = 1024;
        }
    }
}

fn i2_2_8() {
    {
        // クロージャ
        // 普通
        let f = |a, b| a + b;
        let n = f(10, 20);

        //FIXME: 引数にStorageをとるクロージャが作れない..
        // let f3 = |a: &Storage| match a {
        //     Storage::HDD { size: s, .. } => *s += 64,
        //     _ => (),
        // };
        // f3(s)
    }

    // クロージャf
    {
        {
            let mut s = Storage::SSD(512);
            let mut f = || match &mut s {
                Storage::HDD { size: s, .. } => *s += 64,
                _ => (),
            };
            f();
            println!("{:#?}", s);
        }

        {
            // クロージャfの環境
            struct Env_f<'a> {
                storate: &'a mut Storage, // 参照を持つ
            }

            // クロージャfのファットポイント
            struct Clousure_f<'a> {
                ptr_func: fn(),          // 関数へのポインタ
                ptr_env: Box<Env_f<'a>>, // 環境へのポインタ
            }
        }
    }

    // クロージャg
    {
        {
            let mut s = Storage::SSD(512);
            let mut g = move || match &mut s {
                Storage::HDD { size: s, .. } => *s += 64,
                _ => (),
            };
            g();
            // println!("{:#?}", s); // moveしているのでエラー
        }

        {
            // moveを使ったとき、クロージャfとの比較になる
            // クロージャgの環境
            struct Env_g {
                storage: Storage, // 参照ではなく値をもつ
            }

            // クロージャgのファットポインタ
            struct Clousure_g {
                ptr_func: fn(),      // 関数へのポインタ
                ptr_env: Box<Env_g>, // 環境へのポインタ
            }
        }
    }
}

fn i2_2_7() {
    // マクロ
    {
        // assert系
        let a = Some(10);
        assert_matches!(a, Some(_)); // これはマッチする
        assert_matches!(a, Some(10)); // これはマッチする

        // マッチしない
        // assert_matches!(a, Some(12));
    }

    {
        // print!系マクロ
        let n = 56;
        println!("{}", n);
        println!("{:>04}", 56); // 右寄せ4桁 4桁以下の場合0で埋める
        println!("{:#x}", n); // x->16, o->8, b->2進数。#は0x 0o 0bあり
        println!("{:x}", n); // 0xなし16進数
        println!("{:#016x}", n); // 0xあり。16進数を16桁で表し、16桁以下は0で上位の値を埋める
        println!("{:#o}", n);
        println!("{:#b}", n);
        let s = Storage::HDD {
            size: 2048,
            rpm: 7200,
        };
        println!("{:?}", s); // derive(Debug)がないと表示不可
        println!("{:#?}", s); // 同様
    }
}

fn i2_2_6() {
    fn average(v: &[f32]) -> Option<f32> {
        if v.is_empty() {
            return None;
        }

        let mut total = 0.0;
        for n in v {
            total += n;
        }

        Some(total / v.len() as f32)
    }

    let a = [20.0, 10.0];
    let n = average(&a);
    match n {
        Some(value) => {
            println!("{}", value);
        }
        None => {
            println!("error")
        }
    }
    let b: [f32; 0] = [];
}

fn i2_2_5() {
    let spec = PCSpec {
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };

    match &spec {
        PCSpec {
            storage: Storage::SSD(512),
            ..
        } => {
            println!("512GiB SSD")
        }
        PCSpec {
            cpus: 4 | 8,
            memory: m,
            storage: _,
        } => {
            println!("4 or 8 CPUs");
            println!("{}GiB memory", *m);
        }
        PCSpec { memory: m, .. } if *m < 4 => {
            println!("4GiBより少ないメモリ")
        }
        _ => (), // 全パターンマッチ
    };
}

fn i2_2_4() {
    {
        fn sumup_loop(mut n: u64) -> u64 {
            let mut total = 0;

            loop {
                if n == 0 {
                    break;
                }
                total += n;
                n -= 1;
            }
            total
        }

        let a = sumup_loop(24);
        println!("a: {}", a);
    }

    // whileの例
    {
        fn sumup_while(mut n: u64) -> u64 {
            let mut total = 0;
            while n > 0 {
                total += n;
                n -= 1;
            }
            total
        }
    }

    // forの例
    {
        fn sumup_for(n: u64) -> u64 {
            let mut total = 0;
            for x in 0..=n {
                total += x;
            }
            total
        }

        {
            for x in 0..10 {
                println!("x: {x}");
            }
        }
    }

    // ラベルの例
    {
        'main_loop: loop {
            loop {
                println!("label loop");

                break 'main_loop;
            }
        }
    }

    {
        let v = [3, 8, 11, 15];
        let mut result = 0;
        for x in v.iter() {
            if *x % 2 == 0 {
                continue;
            }
            result += *x;
        }
        println!("result: {result}")
    }
}

fn i2_2_3() {
    fn sump(n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            n + sump(n - 1)
        }
    }

    {
        // uだとコンパイルエラー
        let n: i32 = -24;
        let b = if n < 0 {
            println!("nがマイナスの値です");
            n //ここがないとユニット型が返却されるため、コンパイルエラー
        } else {
            n * n
        };

        println!("b の値は{}", b)
    }
}

fn i2_2_2() {
    fn func(a: u32, b: u32) {
        {
            let n: u32 = a + b;
            let m = a + b; // 型推論
        }

        {
            // 未初期化変数読み出しのコンパイルエラー
            // let a: u32;
            // a + 10;
        }

        let n = 10;
        {
            let m = 200;
            let r = m + n;
        }
        // mは別スコープのため、コンパイルエラー
        // let p = m + n;
    }

    // シャドーイングの例
    {
        fn maybe_fail() -> Option<u32> {
            Some(10)
        }

        let result = maybe_fail();
        let result = result.unwrap();
        println!("result: {}", result);
    }
}

fn i2_2_1() {
    // 関数定義
    fn hello() {
        struct Msg {
            msg1: &'static str,
            msg2: &'static str,
        }

        fn print_msg(msg: &Msg) {
            println!("{}{}", msg.msg1, msg.msg2);
        }

        let msg = Msg {
            msg1: "Helllo, ",
            msg2: "world!",
        };

        print_msg(&msg)
    }

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
    let a = add(1, 3);

    println!("a: {a}")
}

fn i2_1_12() {
    // 型変換
    let n: i32 = 100;
    let m: i64 = n as i64;

    let s: String = String::from("abc"); // &str -> Strring
    let s2: String = "abc".into(); // &str -> String
    let s3: String = s2.to_string(); // &str -> String
}

fn i2_1_11() {
    // リンクリストを表すジェネリック型
    {
        enum List<T> {
            Node { data: T, next: Box<List<T>> },
            Nil, // null許可的な？
        }

        let n1 = List::<u32>::Nil;
        let n2 = List::<u32>::Node {
            data: 10,
            next: Box::<List<u32>>::new(n1),
        };
        let n3 = List::Node {
            data: 40,
            // n2型から型推論
            next: Box::new(n2),
        };
    }
    // Option型とResult型
    {
        // 省略
    }
}

fn i2_1_10() {
    {
        enum Dow {
            Sunday,
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
            Saturday,
        }

        let hdd = Storage::HDD {
            size: 512,
            rpm: 7200,
        };

        let ssd = Storage::SSD(512);

        // 構造体の宣言
        struct PCSpec {
            cpus: u16,
            memory: u32,
            storage: Storage,
        }

        let spec = PCSpec {
            cpus: 8,
            memory: 16,
            storage: Storage::SSD(1024),
        };

        println!("{}", spec.cpus);

        // フィールド名の省略
        struct Dim2(u32, u32);
        let d2 = Dim2(10, 20);
        println!("{}", d2.0);

        let r = &spec;
        println!("{}", r.cpus); // 自動的に参照外し
        println!("{}", (*r).cpus); // こう翔が冗長
    }

    // 参照外しの復習
    {
        let a = 24;
        let a_ref = &a;
        println!("a_ref: {}", a_ref); // これも自動的に参照外し(?)
        println!("a_ref: {}", *a_ref); // 参照外し
        println!("a_ref: {:p}", a_ref); // :pつけるとポインタ表示される
    }

    // ジェネリック関数
    {
        fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
            (a, b)
        }

        make_pair::<u8, bool>(40, false);
        make_pair(10, true);
    }

    // 定数を受け取るジェネリック型の例
    {
        struct Buffer<const S: usize> {
            buf: [u8; S],
        }
        let buf = Buffer::<128> { buf: [0; 128] };
    }

    // Option型とResult型
    {
        enum Option<T> {
            Some(T),
        }
    }
}

fn i2_1_9() {
    fn do_it(f: fn(u32, u32) -> u32, a: u32, b: u32) {
        println!("{}", f(a, b))
    }

    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn mul(a: u32, b: u32) -> u32 {
        a * b
    }

    do_it(add, 10, 2);
    do_it(mul, 10, 2)
}

fn i2_1_8() {
    // 0次元のタプルの型は()と表される。ユニット型と呼ばれる。
    fn func() -> () {}

    fn func2() {}
}

fn i2_1_7() {
    // 文字と文字列の型
    // Rustでは文字と文字列は別の型として扱う
    // 文字列スライスを宣言
    let a: &str = " Hello";
    // a += ", world!"; // コンパイルエラー

    let mut b: String = a.to_string();
    b += ", world!   ";

    let c: &str = b.trim();

    println!("c: [{c}]");

    // 複数行の文字列リテラル
    let d = r##"これは
"#複数行の#"
文字列"##;
    println!("{d}")
}

fn i2_1_6() {
    {
        // 配列とマクロ
        let arr: [u32; 4] = [1, 2, 3, 4];
        println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3]);

        let s: &[u32] = &arr[1..3];
        println!("{:?}", s);
        println!("1以上3未満   &arr[1..3]: {:?}", &arr[1..3]);
        println!("1以上3以下   &arr[1..=3]: {:?}", &arr[1..=3]);
        println!("1以上        &arr[1..]: {:?}", &arr[1..]);
        println!("0以上、3未満 &arr[..3]: {:?}", &arr[..3]);

        println!("0以上、3以下 &arr[..3]: {:?}", &arr[..=3]);
        // スライスの範囲外アクセスは、panic
        // println!("0以上、3以下 &arr[..4]: {:?}", &arr[..=4]);

        // 配列の範囲外アクセスは、コンパイルエラー
        // コンパイルエラーは、静的解析で分かるわけではない(Rustの場合赤字で指摘されて、スタックトレース出てない時はコンパイルエラーと判断して良さそう
        //  arr[5];
    }
}

fn i2_1_5() {
    {
        // Rustはそのままの宣言だと、変更不可
        let a0 = 10;
        // a0 = 2; // compile error
        println!("a0: {a0}");

        // ミュートをつけると変更可能になる
        let mut n: u64 = 100; // nを破壊的代入可能として宣言し、100を代入
        n = 1;
        println!("n: {}", n);
    }

    {
        // 不変参照(&)の例
        let mut n: u64 = 100;

        let a: &u64 = &n; // aという参照型(reference type)へ不変参照を代入

        // *a = 200; // 不変参照なんで変更できない
        println!("*a(参照を外した値) = {}, addr(アドレス) = {:p}", *a, a);

        let b: &mut u64 = &mut n; // bという参照型にnの可変参照を代入(可変参照を受け取る時はbも改変参照の型でないとコンパイルエラーになる)
        *b = 200; // bのさしている先に200を破壊的代入
        println!("n = {n}");
    }

    {
        // 可変参照
        // let a: &u64 = &mut n;
    }

    // ミュートをつけると変更可能になる
    //let mut n: u64 = 100; // nを破壊的代入可能として宣言し、100を代入
    // let a: &u64 = &n; // aにnの不変参照を代入
    // println!("*a = {}, addr = {:p}", *a, a); // aを参照外した値(nの値)と、アドレスを表示

    // let b: &mut u64 = &mut n;
    // *b = 200;
    // println!("n = {n}");
}

fn i2_1_3() {
    println!("--- ビットシフト例(矢印の方向にシフトする) ---");

    let n: u8 = 0b0001_1000;
    let m: u8 = n << 2; // 2bit左シフト
    let k = n >> 2; // 2bit右シフト

    println!(
        "元データ: {}, 2ビット左シフト: {}, 2ビット右シフト: {}",
        n, m, k
    ); // 符号なし整数の場合は論理シフト

    println!("--- 算術シフト例 ---");
    let p: i8 = -64; // 0b1100_0000
    let k = p >> 2; // 2bit右算術シフト
    let k2 = p << 2; // 2bit左算術シフト
    println!("p:{}, k:{}, k2: {}", p, k, k2); // 符号あり整数の場合は、算術シフト

    println!("ビット演算");
    // 1 | 1の左1シフト | 568の左2シフト の論理和
    let player: u16 = 1 | // 毒状態
        (1 << 1) | // 1bit: 攻撃力アップ状態
        (568 << 2); // 2bit-15bit: 残り体力

    // a | b: aとbのビット論理和
    // a & B: aとbのビット論理積
    // a ^ b: aとbのビット排他的論理和

    // 1bitの目と1の論理積をとる。毒状態の場合 1&1 = 1 毒でない場合 0&1 = 0となる
    if player & 1 != 0 {
        println!("毒状態");
    }

    // 同様に論理積で、1ビット目から
    if player & (1 << 1) != 0 {
        println!("攻撃力アップ状態");
    }

    // 0xfffc = 0b1111_1111_1111_1100
    // 毒と攻撃力アップ状態を0との論理積で0埋め
    // 右シフトで体力だけの値を残す
    // => 0xfffcのような値はビットマスクと呼ばれる
    // マスクとビット論理積を計算することを「マスクする」と呼ぶ。
    let hp = (player & 0xfffc) >> 2;
    println!("残り体力: {hp}");
}

fn i2_1_2() {
    println!("{}", 1234 + 567);
    println!("{}", 678 - 168);
    println!("{}", 56 * 146);
    println!("{}", 542 / 43);
    println!("{}", 145 % 23);

    println!("{}", 1234 < 567);
    println!("{}", 678 <= 168);
    println!("{}", 56 > 146);
    println!("{}", 572 >= 43);
    println!("{}", 145 == 23);
}

fn i2_1_1() {
    println!("短絡評価");
    println!("{}", a() || b());

    println!("非短絡評価");
}

fn a() -> bool {
    print!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
