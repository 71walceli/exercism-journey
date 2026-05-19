pub fn egg_count(display_value: u32) -> usize {
    let mut display_value = display_value;
    let mut egg_count = 0_usize;
    while display_value > 0 {
        egg_count += (display_value & 1) as usize;
        display_value >>= 1;
    }

    egg_count
}
