#[derive(Debug, PartialEq)]
pub struct UnsignedCounter {
    pub num: usize,
}

impl UnsignedCounter {
    pub fn default_unsigned_counter(&self) -> usize {
        0
    }

    pub fn next_unsigned(counter: &UnsignedCounter) -> Self {
        Self {
            num: counter.num + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned() {
        let test_num = UnsignedCounter { num: 18 };
        assert_eq!(test_num.default_unsigned_counter(), 0);
        assert_eq!(
            UnsignedCounter::next_unsigned(&test_num),
            UnsignedCounter { num: 19 }
        );
    }
}
