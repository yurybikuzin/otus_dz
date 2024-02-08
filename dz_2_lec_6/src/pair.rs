pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

#[test]
fn pair() {
    assert_eq!(default_pair(), (0, 0));
    assert_eq!(pair_scalar_sum((2, 4), (2, 2)), 10);
    assert_eq!(pair_vector_sum((2, 4), (2, 2)), (4, 6));
}
