mod a {
    struct TypeA {
        // a1: a_1::TypeA1, // 子供のプライベートな要素は見えない
        a2: Box<a_2::TypeA2>, // 子供のパブリックな要素は見える
    }

    mod a_1 {
        struct TypeA1 {
            a: Box<super::TypeA>,
            a2: Box<super::a_2::TypeA2>,
        }
    }

    mod a_2 {
        pub struct TypeA2 {
            a: Box<super::TypeA>,
            // a1: super::a_1::TypeA1, // エラー。親の見えないものは見えない
        }
    }
}

mod b {
    pub struct TypeB;

    mod b_1 {
        pub struct TypeB1;
    }

    pub mod b_2 {
        pub struct TypeB2;
    }
}

fn main() {
    //  let a = a::TypeA;
    let b = b::TypeB;
    // let b2 = b::b_2::Typeb2;
    let b2 = b::b_2::TypeB2;
}
