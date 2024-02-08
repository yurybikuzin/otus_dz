#[derive(Debug, PartialEq)]
pub struct Pair(i32, i32);

impl Pair {
    pub fn default(&self) -> Self {
        Self(0, 0)
    }
    pub fn scalar_sum(a: Pair, b: Pair) -> i32 {
        a.0 + a.1 + b.0 + b.1
    }

    pub fn vector_sum(a: &Pair, b: &Pair) -> Self {
        Self(a.0 + b.0, a.1 + b.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair() {
        let test_pair = Pair(5, 7);
        assert_eq!(test_pair.default(), Pair(0, 0));
        assert_eq!(Pair::scalar_sum(Pair(2, 4), Pair(2, 2)), 10);
        assert_eq!(Pair::vector_sum(&Pair(2, 4), &Pair(2, 2)), Pair(4, 6));
    }
}
