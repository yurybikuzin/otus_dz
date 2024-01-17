mod pair;
mod signed;
mod unsigned;
mod vec3;

#[cfg(test)]
mod tests {
    use crate::vec3::default_vec3;
    use crate::vec3::vec3_scalar_sum;
    use crate::vec3::vec3_vector_sum;

    use crate::pair::default_pair;
    use crate::pair::pair_scalar_sum;
    use crate::pair::pair_vector_sum;

    use crate::signed::default_signed_counter;
    use crate::signed::next_signed;
    use crate::signed::prev_signed;

    use crate::unsigned::default_unsigned_counter;
    use crate::unsigned::next_unsigned;

    #[test]
    fn vec3() {
        assert_eq!(default_vec3(), [0; 3]);
        assert_eq!(vec3_scalar_sum([1, 2, 3], [2, 3, 3]), 14);
        assert_eq!(vec3_vector_sum([1, 2, 3], [2, 3, 3]), [3, 5, 6]);
    }

    #[test]
    fn pair() {
        assert_eq!(default_pair(), (0, 0));
        assert_eq!(pair_scalar_sum((2, 4), (2, 2)), 10);
        assert_eq!(pair_vector_sum((2, 4), (2, 2)), (4, 6));
    }

    #[test]
    fn signed() {
        assert_eq!(default_signed_counter(), 0);
        assert_eq!(next_signed(3), 4);
        assert_eq!(prev_signed(-2), -3);
    }

    #[test]
    fn unsigned() {
        assert_eq!(default_unsigned_counter(), 0);
        assert_eq!(next_unsigned(19), 20);
    }
}
