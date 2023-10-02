pub fn rand_even() -> u32 {
    rand::random::<u32>() & !1
}
