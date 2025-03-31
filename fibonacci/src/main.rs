fn fibonacci(val: u64) -> u64 {
    if val == 0 {
        return 0;
    }
    let mut curr_nb: u64 = 1;
    let mut last_nb: u64 = 0;
    let mut _temp_nb: u64 = 0;

    for _nb in 1..val {
        _temp_nb = curr_nb;
        curr_nb = curr_nb + last_nb;
        last_nb = _temp_nb;
    }
    curr_nb
}

fn main() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);

    // Small values
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);

    // Medium values
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(15), 610);
    assert_eq!(fibonacci(20), 6765);

    // Larger values (be careful with overflow for u64/i32)
    assert_eq!(fibonacci(30), 832040);
    assert_eq!(fibonacci(40), 102334155);

    assert_eq!(fibonacci(50), 12586269025);
    assert_eq!(fibonacci(90), 2880067194370816120); // Requires u64 or larger
}
