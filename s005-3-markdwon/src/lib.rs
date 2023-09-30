//! # 第一見出し
//!
//! テキストを書く
//!
//! ## 第二見出し
//!
//! ### 第三見出し
//!
//! - 箇条書き 1
//! - 箇条書き 2
//!
//! 1. 番号付きリスト1
//! 2. 番号付きリスト2
//!
//! > 引用
//! > 文字列
//!
//! [KSPUB](https://www.kspub.co.jp/)
//!
//! `println!("Hello, world!");`
//!
//! ```
//! println!("Hello, world");
//! ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/// my_func は私独自の関数です。
///
/// # 利用例
///
/// ```
/// use markdwon::my_func;
/// let n = my_func().unwrap();
/// ```
pub fn my_func() -> Option<u32> {
    Some(100)
}
