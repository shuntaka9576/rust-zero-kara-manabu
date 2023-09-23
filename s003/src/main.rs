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

        // let v = xy.get_vec();
        // xy.update(v); // `xy`は借用されているためコンパイルエラー

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
