pub fn nth(nth: u32) -> u32 {
    let mut candidates = Vec::new();
    candidates.push(2u32);
    candidates.push(3u32);
    candidates.push(5u32);
        
    if nth < candidates.len() as u32 {
        return candidates[nth as usize];
    }
    
    let mut _nth = candidates.len() as u32 -1;
    let mut _candidate = *candidates.last().unwrap();
    let mut is_prime = true;
    while _nth != nth {
        for c in candidates.iter().take_while(|_c| **_c <= _candidate.isqrt()) {
            let is_multiple = _candidate.is_multiple_of(*c);
            is_prime = !is_multiple;
            if is_multiple {
                break;
            }
        }
        
        if is_prime && candidates.binary_search(&_candidate).is_err() {
            candidates.push(_candidate);
            _nth += 1u32;   
        } else {
            _candidate += 2u32;
        }        
    }
    
    _candidate
}
 