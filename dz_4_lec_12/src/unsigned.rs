#[derive(Debug, PartialEq)]
pub struct UnsignedCounter {
    pub num: usize,
}

impl UnsignedCounter {
    pub fn default_unsigned_counter(&self) -> Self {
        Self { num: 0 }
    }

    pub fn next_unsigned(&self) -> Self {
        Self { num: &self.num + 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned() {
        let test_num = UnsignedCounter { num: 18 };

        assert_eq!(
            test_num.default_unsigned_counter(),
            UnsignedCounter { num: 0 }
        );
        assert_eq!(
            UnsignedCounter::next_unsigned(&test_num),
            UnsignedCounter { num: 19 }
        );
        assert_eq!(test_num.next_unsigned(), UnsignedCounter { num: 19 });
    }
}
