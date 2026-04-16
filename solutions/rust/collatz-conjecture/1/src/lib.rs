pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    
    let mut n = n;
    let mut steps = 0;

    while n > 1 {
        steps += 1;
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n = n*3 +1;
        }
    }

    Some(steps)
}
