//! 正規表現の式をパースし、抽象構文木に変換
use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

pub enum ParserError {
    InvalidEscape(usize, char),
}

// fn take() {
//     let mut n = Some(10);
//     let v = take(&mut n);
//     println("n = {:?}, v = {:?}", n, v);
// }
