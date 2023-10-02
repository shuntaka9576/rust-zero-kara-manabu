use s005_4_3; // my_great_lib

#[test]
fn test_rand_even() {
    for _ in 0..100 {
        let result = s005_4_3::rand_even();
        assert_eq!(result % 2, 0);
    }
}
