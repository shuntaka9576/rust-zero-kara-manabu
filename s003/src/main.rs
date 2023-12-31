use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    error::Error,
    fmt::{write, Debug, Display, Formatter},
    fs::File,
    io::{Read, Write},
    ops::Mul,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // スタックメモリ
    s_3_1();

    // 所有権
    s_3_2();

    // ライフタイム
    s_3_3();

    // ライフタイムサブタイピング
    s_3_3_1();

    // 借用
    s_3_4();

    // 可変参照として借用
    println!("--- 3.4.2 ---");
    s_3_4_2();
    println!("---");

    println!("--- 3.5 ---");
    // s_3_5(); // 実行時間がかかるためコメントアウト
    println!("---");

    println!("--- 4.1 ---");
    s_4_1();
    println!("---");
    println!("--- 4.2 ---");
    s_4_2();
    println!("---");
    println!("--- 4.2 pra ---");
    s_4_2_pra();
    println!("---");
    println!("--- 4.3 ---");
    // s_4_3(); // watchオプションでファイル変化を検知してしまうため(多分)
    println!("---");
    println!("--- 4.4 ---");
    s_4_4();
    println!("---");
    println!("--- 4.5 ---");
    s_4_5();
    println!("--- 4.6 ---");
    s_4_6();
    println!("---");
    println!("--- 4.7 ---");
    s_4_7();
    println!("---");
}

fn s_4_7() {
    // 存在型

    {
        fn closure() -> impl Fn(i32) -> i32 {
            |x| x * x
        }
    }

    {
        let f = |x: i32| x * x;
        let g = |x: i32| x * x;

        fn get_type_id<T: Any>(_: &T) -> TypeId {
            TypeId::of::<T>()
        }

        println!("{}", get_type_id(&f) == get_type_id(&g)); // falseになる
    }

    {}
}

fn s_4_6() {
    {
        trait Location {
            fn address(&self) -> &str;
        }

        trait Person {
            fn name(&self) -> &str;
        }

        // LocationとPersonのスーパートレイト
        trait House: Location + Person {}

        fn print_house_info(house: &dyn House) {
            println!("所有者: {}", house.name());
            println!("住所: {}", house.address());
        }

        struct MyHouse {
            owner: String,
            address: String,
        }

        impl Location for MyHouse {
            fn address(&self) -> &str {
                &self.address
            }
        }

        impl Person for MyHouse {
            fn name(&self) -> &str {
                &self.owner
            }
        }

        impl House for MyHouse {}

        let my_house = MyHouse {
            owner: "かぐや姫".to_string(),
            address: "ムーンベース3丁目1番地".to_string(),
        };

        print_house_info(&my_house)
    }
}

fn s_4_5() {
    {
        trait Foo {
            fn foo(&self);
        }

        struct Bar;
        impl Foo for Bar {
            fn foo(&self) {
                println!("Bar:foo");
            }
        }

        struct Buzz;
        impl Foo for Buzz {
            fn foo(&self) {
                println!("Buzz:foo");
            }
        }

        // コンパイル時にTが決定される
        fn call_foo_static<T: Foo>(arg: &T) {
            arg.foo();
        }

        // 実行時に呼び出し先が決定される
        fn call_foo_dynamic(arg: &dyn Foo) {
            arg.foo();
        }

        let bar = Bar;
        let buzz = Buzz;

        // 静的ディスパッチ
        call_foo_static(&bar);
        call_foo_static(&buzz);

        // 動的ディスパッチ
        call_foo_dynamic(&bar);
        call_foo_dynamic(&buzz);
    }

    {
        // 定義ジャンプして、実装しないといけないトレイトを確認する
        // pub trait Error: Debug + Display {
        //     fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
        //     fn backtrace(&self) -> Option<&Backtrcae> { ... }
        //     fn description(&self) -> &str { ... }
        //     fn caause(&self) -> Option<&dyn Error> { ... }
        // }

        // Error: Debug + Display
        // Displayで、Errorトレイト実装するには、DebugとDeisplayトレイトを実装する必要がある
        // デフォルト実装されているものは自分dね実装する必要はない

        #[derive(Debug)]
        struct ErrorA;

        impl Display for ErrorA {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error A")
            }
        }

        impl Error for ErrorA {} // Error を ErrorAに実装

        // エラーBを定義
        #[derive(Debug)]
        struct ErrorB;

        impl Display for ErrorB {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error B")
            }
        }

        impl Error for ErrorB {}
        fn error_a() -> Result<(), ErrorA> {
            Err(ErrorA)
        }

        fn error_b() -> Result<(), ErrorB> {
            Err(ErrorB)
        }

        fn error_ab() -> Result<(), Box<dyn std::error::Error>> {
            error_a()?;
            error_b()?;
            Ok(())
        }

        // 存在型は静的ディスパッチであるため、複数の型を返すような関数を定義するとコンパイルエラーとなる
        // 複数の型を返却する場合、前述のように動的ディスパッチを利用するのが良い
        fn error_ab_impl(flag: bool) -> impl std::error::Error {
            ErrorA
            // これだとコンパイルエラー
            // if flag {
            //     ErrorA
            // }
            // else {
            //     ErrorB
            // }
        }
    }
}

// トレイト制約
fn s_4_4() {
    {
        {
            /// T: Mul<Output = T> + Copy
            /// 型引数Tが std::ops::Mulトレイトと、Copyトレイトを実装していなければならないという制約
            /// Output = Tは、std::ops::Mulトレイトの関連型Outputの型はT型であるという意味
            fn square<T>(x: T) -> T
            where
                T: Mul<Output = T> + Copy,
            {
                x * x
            }

            // pub trait Mul<Rhs = Self> {
            // type Output;
            // fn mul(self, rhs: Rhs) -> Self::Output;
            // }
        }

        // 別の書き方
        {
            // 型引数Tの後にコロンでトレイト制約を記述
            fn square<T: Mul<Output = T> + Copy>(x: T) -> T {
                x * x
            }
        }
    }

    {
        // Arcで複数のスレッド間で参照を受け渡しする例
        {
            let n = Arc::new(10);
            thread::spawn(move || println!("{n}"));
        }

        // Rcを複数のスレッド間で共有してコンパイルエラーとなる例
        {
            let n = Rc::new(10);

            // コンパイルエラー
            // thread::spawn(|| {
            //     println!("{n}");
            // });
            // error[E0277]: `Rc<i32>` cannot be shared between threads safely
            //    --> src/main.rs:97:27
            //     |
            // 97  |               thread::spawn(|| {
            //     |  _____________-------------_^
            //     | |             |
            //     | |             required by a bound introduced by this call
            // 98  | |                 println!("{n}");
            // 99  | |             });
            //     | |_____________^ `Rc<i32>` cannot be shared between threads safely
            //         }
            //     }
        }

        // Rcを別の型で包んでみる
        // 基本コンパイルエラーになる。Rustだとコンパイル時に検出できるため、比較的容易に安全な並行プログラミングが可能
        {
            // ---
            let n = Arc::new(Rc::new(10));
            // thread::spawn(move || println!("{n}"))

            // ---
            let n2 = Arc::new(Mutex::new(Rc::new(10)));
            // thread::spawn(move || {
            //     let n = n2.lock().unwrap();
            //     println!("{n}")
            // });
        }
    }
}

fn s_4_3() {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }

    impl<T> List<T> {
        fn new() -> List<T> {
            List::Nil
        }

        fn cons(self, data: T) -> List<T> {
            List::Node {
                data,
                next: Box::new(self),
            }
        }

        fn iter<'a>(&'a self) -> ListIter<'a, T> {
            ListIter { elm: self }
        }
    }

    struct ListIter<'a, T> {
        elm: &'a List<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.elm {
                List::Node { data, next } => {
                    self.elm = next;
                    Some(data)
                }
                List::Nil => None,
            }
        }
    }

    // Listのシリアライズとデシリアライズ
    {
        let list = List::new().cons(1).cons(2).cons(3);

        for i in list.iter() {
            println!("check: {i}")
        }

        // シリアライズ
        let js = serde_json::to_string(&list).unwrap();
        println!("JSON: {} bytes", js.len());
        println!("{js}");

        let yml = serde_yaml::to_string(&list).unwrap();
        println!("YAML: {} bytes", yml.len());
        println!("{yml}");

        let msgpack = rmp_serde::to_vec(&list).unwrap();
        println!("MessagePack: {} bytes", msgpack.len());

        // デシリアライズ
        let list = serde_json::from_str::<List<i32>>(&js).unwrap();
        println!("{:?}", list);

        let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
        println!("{:?}", list);

        let list = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
        println!("{:?}", list);
    }

    // ファイルへの書き出し
    {
        let list = List::new().cons(1).cons(2).cons(3);
        let yml = serde_yaml::to_string(&list).unwrap();

        // ファイルへの書き出し
        let path = Path::new("test.yml");
        let mut f = File::create(path).unwrap();
        f.write_all(yml.as_bytes()).unwrap();
    }

    // ファイルからの読み出し
    {
        println!("読み出し");
        let path = Path::new("test.yml");
        let mut f = File::open(path).unwrap();
        let mut yml = String::new();
        f.read_to_string(&mut yml).unwrap();

        // YAMLからデシリアライズ
        let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
        println!("{:?}", list);
    }
}

fn s_4_2() {
    #[derive(Debug, Clone)]
    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }

    impl<T> List<T> {
        // なぜNil?
        // おそらく即時でList<T>型をを返却できるから
        // 後でconsを使っていく設計(?)
        fn new() -> List<T> {
            List::Nil
        }

        /// リストを消費して、そのリストの先頭に引数dataを追加したリストを返す
        fn cons(self, data: T) -> List<T> {
            List::Node {
                data,
                next: Box::new(self),
            }
        }

        /// 不変イテレータを返す
        fn iter<'a>(&'a self) -> ListIter<'a, T> {
            ListIter { elm: self }
        }
    }

    // このライフタイム解釈がわからない(?)
    struct ListIter<'a, T> {
        elm: &'a List<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        // typeキーワードで関連型(associated type)を定義できる
        type Item = &'a T; // イテレータが指す要素の型

        fn next(&mut self) -> Option<Self::Item> {
            match self.elm {
                List::Node { data, next } => {
                    self.elm = next;
                    Some(data)
                }
                List::Nil => None,
            }
        }
    }

    {
        let list = List::new().cons(0).cons(1).cons(2);

        for x in list.iter() {
            println!("{x}");
        }

        println!();

        let mut it = list.iter();
        println!("{:?}", it.next().unwrap());
        println!("{:?}", it.next().unwrap());
        println!("{:?}", it.next().unwrap());
    }
}

fn s_4_1() {
    struct ImaginaryNumber {
        real: f64,
        img: f64,
    }

    impl Display for ImaginaryNumber {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{} + {}i", self.real, self.img)
        }
    }

    let n = ImaginaryNumber {
        real: 3.0,
        img: 4.0,
    };
    println!("{n}");
}

fn s_3_5() {
    // Arcの利用
    {
        let v = Arc::new(vec![1, 2, 3]);
        let w = v.clone(); // 参照カウント = 2
        let z = v.clone(); // 参照カウント = 3
    }

    // ミューテックスの利用
    {
        let x = Arc::new(Mutex::new(100_000)); // Mutex型の値を生成
        let x2 = x.clone(); // 参照カウンタをインクリメント

        let h1 = std::thread::spawn(move || {
            // スレッド
            let mut guard = x.lock().unwrap();
            *guard -= 20_000; // ガードを参照して保護対象データにアクセス
        });

        // x1にすると所有権?の影響かコンパイルエラーになる。
        // 複数スレッド間で値を共有する場合、cloneしてスマートポインタを用意するのが良さそう？
        let h2 = std::thread::spawn(move || {
            let mut guard = x2.lock().unwrap();
            *guard -= 30_000; // ガードを参照して保護対象データにアクセス
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }

    {
        // 借用と排他制御の類似性
        // ---借用
        // * ある時点で、書き込み可能な状態にある変数(参照を含む)は、最大1つである。
        // * ある時点で、不変参照が1つ以上存在する場合、書き込み可能な状態にある変数(参照を含む)は1つも存在しない
        // ---
        // ---RW
        // * ある時点で、ロック獲得中のライターは最大1つである。
        // * ある時点で、ロック獲得中のリーダーが1つ以上存在する場合、ロック獲得中のライターは1つも存在しない
        // ---
        let mut gallery = BTreeMap::new();
        gallery.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
        gallery.insert("ミュシャ", "黄道十二宮");

        // RwLockとArcを利用して共有可能に
        let gallery = Arc::new(RwLock::new(gallery));

        let mut hdls = Vec::new();
        for n in 0..3 {
            // 客を表すスレッドを生成
            let gallery = gallery.clone(); // 参照カウンタをインクリメント
            let hdl = std::thread::spawn(move || {
                for _ in 0..8 {
                    {
                        // readで取得したguard経由で書き込みはできない
                        let guard = gallery.read().unwrap();
                        if n == 0 {
                            // 美術館の展示内容を表示
                            for (key, value) in guard.iter() {
                                print!("{key}:{value}");
                            }
                            println!();
                        }
                    }
                    sleep(Duration::from_secs(1))
                }
            });
            hdls.push(hdl)
        }

        // 美術館スタッフ
        let staff = std::thread::spawn(move || {
            for n in 0..4 {
                // 展示内容を入れ替え
                if n % 2 == 0 {
                    let mut guard = gallery.write().unwrap(); // ライトロック
                    guard.clear();
                    guard.insert("ゴッホ", "星月夜");
                    guard.insert("エッシャー", "滝");
                } else {
                    let mut guard = gallery.write().unwrap(); // ライトロック
                    guard.clear();
                    guard.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
                    guard.insert("ミュシャ", "黄道十二宮");
                }
                sleep(Duration::from_secs(2));
            }
        });

        for hdl in hdls {
            hdl.join().unwrap();
        }
        staff.join().unwrap();
    }
}

fn s_3_4_2() {
    // 可変参照としての借用
    // * ある時点で書き込み可能な状態にある変数(参照を含む)は最大1つである。
    // * ある時点で不変参照が1つ以上存在する場合、書き込み可能な状態にある変数(参照を含む)は1つも存在しない

    {
        let mut a = 10; // オリジナル変数
        let b = &a; // &借用

        // let c = &mut a; // 遷移不可 // &mut 借用
        // println!("{a} {b} {c}");
    }

    {
        let a = 10;
        let b = &a; // &借用(コピーセマンティクス)
        let c = b; // &借用コピー
        let d = &a; // &借用
        println!("{a}, {b}, {c}, {d}");
    }

    // &借用がコピーセマンティクスで&mut 借用がムーブセマンティクスではない例外
    {
        fn bar(x: &mut i32) {
            *x += 1;
        }

        let mut a = 10;
        let b = &mut a;
        // 関数呼び出し場合、ムーブについては考えないことでほとんどの場合うまくいく
        bar(b); // bは関数にムーブされるため、状態RW借用の遷移図的には終了だが..
        *b += 10; // bの操作をすることが可能。
        println!("b: {b}")
    }

    // 構造体のフィールドを借用すると、その構造体本体も借用されてしまう
    // 本誌誤植(See. https://github.com/ytakano/rust_zero/issues/21)
    {
        #[derive(Debug)]
        struct XY {
            x: Vec<i32>,
            y: Vec<i32>,
            selector: bool,
            scaler: i32,
        }

        impl XY {
            /// `selector`の応じて、`x`か`y`を返す
            fn get_vec(&mut self) -> &mut [i32] {
                if self.selector {
                    &mut self.x
                } else {
                    &mut self.y
                }
            }

            /// `v`になんらかの定型処理を行う
            fn update(&mut self, v: &mut [i32]) {
                for elm in v.iter_mut() {
                    *elm *= self.scaler;
                }
            }
        }

        let mut xy = XY {
            x: vec![1, 2, 3],
            y: vec![4, 5, 6],
            selector: true,
            scaler: 3,
        };

        let v = xy.get_vec();
        // xy.update(v); // `xy`は借用されているためコンパイルエラー

        // get_vec()で返される可変参照がvに借用されているため、update()で必要な&mut selfが借用できない

        println!("{:?}", xy);
    }
}

fn s_3_4() {
    {
        let a = 10; // ref_count == 0
        {
            let b = &a; // ref_count == 1
            let c = &a; // ref_count == 2
            let d = b; // ref_count == 3
        }
        // ref_count = 0
    }

    {
        // 無効な参照の参照カウント
        let a;
        {
            let b = 10; // ref_count == 0
            a = &b; // ref_count == 1
        } // bがスタックから削除
          // println!("{}", a)
    }
}

fn s_3_3_1() {
    fn add<'a>(x: &'a mut i32, y: &'a i32) {
        *x += *y;
    }

    let mut x = 10;
    {
        let y = 20;
        add(&mut x, &y);
    }
    println!("{x}");
}

fn s_3_3() {
    {
        let a;
        {
            let b = 10;
            a = &b;
        } // <-- ここでbの参照が抜ける

        // コンパイルエラー
        // println!("{}", a);
    }

    {
        let a;
        {
            let b = 10;
            a = &b;
            println!("{}", a) // 字句ライフタイムでは、ブロックが生存期間だったが、非字句ライフタイムではaが最後に利用されるまでとなる(意味的な解釈が行われる)
                              // この仕様はRust2018で導入
        }
    }

    // ライフタイム指定子
    {
        let a: i32 = 10;
        let b: &i32 = &a;

        fn square<'a>(x: &'a i32) -> i32 {
            x * x
        }
        square(b);

        struct Foo<'a> {
            x: &'a i32,
        }

        // Foo<'a>は参照を持つ構造体
        // フィールドxはのライフタイムは'aであると指定している。
        // 構造体や列挙型のフィールドに参照を含める場合は、必ずライフタイム指定子を用いる必要がある。
        Foo { x: &a };
    }
}

fn s_3_2() {
    {
        struct H2O {}
        struct O2 {}
        struct H2 {}

        /// 水分子を2個、酸素ぶんしを1個消費して
        fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
            (H2O {}, H2O {})
        }

        let h2_1 = H2 {};
        let h2_2 = H2 {};
        let o2 = O2 {};

        let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2); // 燃焼

        // コンパイルエラー。すでに消費した分子は使えない
        // let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2); // 燃焼
    }

    // 支出と収入
    {
        struct Coin {}

        let a = Coin {};
        let b = a;
        let c = b;
        // コンパイルエラー
        // let d = a;
    }

    // コピーされる型
    {
        let a = 10;
        let b = 20;
        let c = a + b; // aとbを利用
        let d = a * b; // aとbを再度利用
    }
}

fn s_3_1() {
    {
        let a = 10;
        let b = 20; // 地点 1

        {
            let c = 30;
            let d = 40; // 地点 2
            n(); // 地点 4
        }
        // 地点 5
        fn n() {
            let e = 50;
            let f = 60;
        }
    }
}

// ---練習---
// s_4_2(イテレーター実装)を何も見ないで書いてみるところ
fn s_4_2_pra() {}
