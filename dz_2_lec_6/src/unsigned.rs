pub type UnsignedCounter = usize;

#[allow(dead_code)]
pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

#[allow(dead_code)]
pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}
