// 5.2.2.
mod b {
    pub struct TypeB;
    mod b_1 {
        pub struct TypeB1 {
            pub n: usize,
            m: usize,
        }

        impl TypeB1 {
            fn g(&self) {}
            pub fn h(&self) {}
        }

        fn f1(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            println!("{}", p.m);
            p.g();
            p.h();
        }
    }
    pub mod b_2 {
        pub struct TypeB2;

        fn f2(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            // println!("{}", p.m);
            // p.g();
            p.h();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
