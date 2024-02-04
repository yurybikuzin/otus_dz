#[derive(Debug, PartialEq)]
pub struct Pair(i32, i32);

impl Pair {
    pub fn default_pair(&self) -> Self {
        Self(0, 0)
    }
    pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
        a.0 + a.1 + b.0 + b.1
    }

    pub fn pair_vector_sum(a: &Pair, b: &Pair) -> Self {
        Self(a.0 + b.0, a.1 + b.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair() {
        let test_pair = Pair(5, 7);
        assert_eq!(test_pair.default_pair(), Pair(0, 0));
        assert_eq!(Pair::pair_scalar_sum(Pair(2, 4), Pair(2, 2)), 10);
        assert_eq!(Pair::pair_vector_sum(&Pair(2, 4), &Pair(2, 2)), Pair(4, 6));
    }
}
