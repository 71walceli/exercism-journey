pub fn is_leap_year(year: u64) -> bool {
    matches!(
        (year.is_multiple_of(4), year.is_multiple_of(100), year.is_multiple_of(400)),
        (true, false, false) | (true, true, true)
    )
}
