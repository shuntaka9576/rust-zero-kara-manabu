use std::{
    error::Error,
    fmt::{self, write, Display},
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

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char), // 誤ったエスケープシーケンス
    InvalidRightParen(usize),   // 開き括弧なし
    NoPrev(usize),              // + | * ? の前に式がない
    NoRightParen,               // 閉じ括弧なし
    Empty,                      // 空のパターン
}

enum PSQ {
    Plus,
    Star,
    Question,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(
                    f,
                    "ParseError: invalid right parenthesis: pos = {pos}, char = '{c}'"
                )
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NoRightParen => {
                write!(f, "ParseError: no right parenthesis")
            }
            ParseError::Empty => write!(f, "ParseError: empty expression"),
        }
    }
}

// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);

            Err(err)
        }
    }
}

/// +, *, ?をASTに変換
///
/// 後置記法で、+、*、?の前にパターンがない場合はエラー
///
/// 例: *ab、abc|+などはエラー
#[rustfmt::skip]
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() { // (1)
        let ast = match ast_type { // (2)
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast); // (3)
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos)) // (4)
    }
}

/// Orで結合された複数の式をASTに変換
///
/// 例えば、abc|def|ghiは、AST::Or("abc", AST::Or("def", "ghi")) というASTになる
#[rustfmt::skip]
fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 { // (1)
        let mut ast = seq_or.pop().unwrap();
        seq_or.reverse(); // (2)
        for s in seq_or {
            ast = AST::Or(Box::new(s), Box::new(ast));
        }
        Some(ast)
    } else {
        seq_or.pop() // (3)
    }
}

pub fn parse(expr: &str) -> Result<AST, ParseError> {
    // 内部状態を表現するための型
    // Char 状態: 文字列処理中
    // Escape 状態: エスケープシーケンス処理中
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq = Vec::new();
    let mut seq_or = Vec::new();
    let mut stack = Vec::new();
    let mut state = ParseState::Char;

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, PSQ::Star, i)?,
                '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                '(' => {
                    let prev = take(&mut seq);
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or));
                }
                ')' => {
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        if !seq.is_empty() {
                            seq_or.push(AST::Seq(seq));
                        }

                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }

                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        // "abc)"のように、開き括弧がないのに閉じ括弧がある場合はエラー
                        return Err(Box::new(ParseError::InvalidRightParen(i)));
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        return Err(Box::new(ParseError::NoPrev(i)));
                    } else {
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
            }
        }
    }

    if !stack.is_empty() {
        return Err(Box::new(ParseError::NoRightParen));
    }

    if !seq.is_empty() {
        seq_or.push(AST::Seq(seq));
    }

    if let Some(ast) = fold_or(seq_or) {
        Ok(ast)
    } else {
        Err(Box::new(ParseError::Empty))
    }
}
