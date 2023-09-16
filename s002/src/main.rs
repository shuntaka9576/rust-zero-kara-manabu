fn main() {
    println!("--- 2.1.1 ---");
    i2_1_1();
    println!("---");
    println!("--- 2.1.2 ---");
    i2_1_2();
    println!("---");
    println!("--- 2.1.3 ---");
    i2_1_3();
    println!("---");
}
fn i2_1_3() {
    println!("--- ビットシフト例(矢印の方向にシフトする) ---");

    let n: u8 = 0b0001_1000;
    let m: u8 = n << 2; // 2bit左シフト
    let k = n >> 2; // 2bit右シフト

    println!(
        "元データ: {}, 2ビット左シフト: {}, 2ビット右シフト: {}",
        n, m, k
    ); // 符号なし整数の場合は論理シフト

    println!("--- 算術シフト例 ---");
    let p: i8 = -64; // 0b1100_0000
    let k = p >> 2; // 2bit右算術シフト
    let k2 = p << 2; // 2bit左算術シフト
    println!("p:{}, k:{}, k2: {}", p, k, k2); // 符号あり整数の場合は、算術シフト

    let player: u16 = 1 | (1 << 1) | (568 << 2);

    println!("1 | 1 << 1({}) | 568 << 2({})", 1 << 1, 568 << 2);
    if player & 1 != 0 {
        println!("毒状態");
    }
}

fn i2_1_2() {
    println!("{}", 1234 + 567);
    println!("{}", 678 - 168);
    println!("{}", 56 * 146);
    println!("{}", 542 / 43);
    println!("{}", 145 % 23);

    println!("{}", 1234 < 567);
    println!("{}", 678 <= 168);
    println!("{}", 56 > 146);
    println!("{}", 572 >= 43);
    println!("{}", 145 == 23);
}

fn i2_1_1() {
    println!("短絡評価");
    println!("{}", a() || b());

    println!("非短絡評価");
}

fn a() -> bool {
    print!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
