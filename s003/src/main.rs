fn main() {
    // スタックメモリ
    s_3_1();

    // 所有権
    s_3_2();

    // ライフタイム
    s_3_3();
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
