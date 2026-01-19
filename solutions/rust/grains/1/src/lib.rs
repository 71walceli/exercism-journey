pub fn square(s: u32) -> u64 {
    if !(0..65).contains(&s) {
        panic!("Invalid number of squares")
    }
    
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    u64::MAX
}
