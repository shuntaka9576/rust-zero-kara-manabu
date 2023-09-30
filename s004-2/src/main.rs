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

mod c {
    mod c_1_outer {
        pub mod c_1_inner {
            pub(crate) struct TypeC1;
            pub(super) struct TypeC2;
            pub(in crate::c::c_1_outer) struct TypeC3;
            pub(self) struct TypeC4;
        }

        fn f() {
            let p1 = c_1_inner::TypeC1;
            let p2 = c_1_inner::TypeC2;
            let p3 = c_1_inner::TypeC3;
        }
    }

    fn g() {
        let p1 = c_1_outer::c_1_inner::TypeC1;
    }
}

mod d {
    pub struct TypeD;
}

mod e {
    pub use crate::d::TypeD;
}

fn main() {
    let e = e::TypeD;
}
