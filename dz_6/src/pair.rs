pub type Pair = (i32, i32);

#[allow(dead_code)]
pub fn default_pair() -> Pair {
    (0, 0)
}

#[allow(dead_code)]
pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[allow(dead_code)]
pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}
