#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_func() {
        assert_eq!(my_func(), Some(100))
    }
}

/// ```
/// use markdown::my_func;
/// let n = my_func().unwarp();
/// ```
pub fn my_func() -> Option<u32> {
    Some(100)
}

// -- 異常系テスト ---
pub fn pred(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    #[should_panic] // パニックすべき
    fn test_pred() {
        pred(0).unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}
