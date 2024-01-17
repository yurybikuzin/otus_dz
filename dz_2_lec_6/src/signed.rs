pub type SignedCounter = isize;

#[allow(dead_code)]
pub fn default_signed_counter() -> SignedCounter {
    0
}

#[allow(dead_code)]
pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

#[allow(dead_code)]
pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}
