pub type SignedCounter = isize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[test]
fn signed() {
    assert_eq!(default_signed_counter(), 0);
    assert_eq!(next_signed(3), 4);
    assert_eq!(prev_signed(-2), -3);
}
