fn main() {
    let x: i32 = 10;
    let y:i32 = 20;
    let z = mul(x, y);

    println!("z = {z}");
}

fn mul(x: i32, y: i32) -> i32 {
    x*y
}
