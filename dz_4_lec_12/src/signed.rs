#[derive(Debug, PartialEq)]
pub struct SignedCounter {
    pub num: isize,
}

impl SignedCounter {
    pub fn default_signed_counter(&self) -> Self {
        Self { num: 0 }
    }

    pub fn next_signed(&self) -> Self {
        Self { num: &self.num + 1 }
    }

    pub fn prev_signed(&self) -> Self {
        Self { num: &self.num - 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed() {
        let test_num = SignedCounter { num: 4 };

        assert_eq!(test_num.default_signed_counter(), SignedCounter { num: 0 });
        assert_eq!(
            SignedCounter::next_signed(&test_num),
            SignedCounter { num: 5 }
        );
        assert_eq!(
            SignedCounter::prev_signed(&test_num),
            SignedCounter { num: 3 }
        );
    }
}
