#[derive(Debug, PartialEq)]
pub struct SignedCounter {
    pub num: isize,
}

impl SignedCounter {
    pub fn default_signed_counter(&self) -> isize {
        0
    }

    pub fn next_signed(counter: &SignedCounter) -> isize {
        counter.num + 1
    }

    pub fn prev_signed_1(counter: &SignedCounter) -> isize {
        counter.num - 1
    }

    pub fn prev_signed_2(counter: &SignedCounter) -> Self {
        Self {
            num: counter.num - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed() {
        let test_num = SignedCounter { num: 4 };
        assert_eq!(test_num.default_signed_counter(), 0);
        assert_eq!(SignedCounter::next_signed(&test_num), 5);
        assert_eq!(SignedCounter::prev_signed_1(&test_num), 3);
        assert_eq!(
            SignedCounter::prev_signed_2(&test_num),
            SignedCounter { num: 3 }
        );
    }
}
