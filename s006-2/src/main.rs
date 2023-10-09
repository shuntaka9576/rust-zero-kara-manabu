fn main() {
    // 文字列をバイナリで表示する
    let s = "aはAの小文字".as_bytes();
    for i in s {
        println!("{:x}", i);
    }
    println!();
}

// 61 -> a
// e3 -> は(3バイト)
// 81
// af
// 41 -> A
// e3 -> の(3バイト)
// 81
// ae
// e5 -> 小(3バイト)
// b0
// 8f
// e6 -> 文(3バイト)
// 96
// 87
// e5 -> 字(3バイト)
// ad
// 97