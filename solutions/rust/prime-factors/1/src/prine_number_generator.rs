pub fn verify_divisibility<'a, I>(primes: I, candidate: u64) -> bool 
where 
    I: Iterator<Item = &'a u64>
{
    for _prime in primes {
        let _prime = *_prime;
        if candidate.is_multiple_of(_prime) && candidate != _prime {
            return false;
        }
    }
    
    true
}

pub fn generate_primes(n: u64) -> Vec<u64> {
    let mut primes = vec!(2u64, 3u64);
    
    let last_prime = *primes.last().unwrap() +2;
    for candidate in last_prime..=n.isqrt() {
        if verify_divisibility(primes.iter(), candidate) 
            && primes.binary_search(&candidate).is_err() 
        {
            primes.push(candidate);
        }
    }
    
    primes.into_iter().filter(|_prime| *_prime <= n.isqrt()).collect()
}
