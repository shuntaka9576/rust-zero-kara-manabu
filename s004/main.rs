fn main() {
    // module
    {
        mod a {

            struct TyepA;
            mod a_1 {
                struct TyepA1;
            }
            mod a_2 {
                struct TyepA2;
            }
        }

        mod b {
            struct TypeB;
            mod b_1 {
                struct TypeB1;
            }
            mod b_2 {
                struct TypeB2;
            }
        }
    }

    // 可視性

    mod a {
        struct TypeA {
            a2: Box<a_2::TypeA2>,
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
                a1: super::a_1::TypeA1,
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
        let b = b::TypeB;
        let b2 = b::b_2::Typeb2;
    }
}
