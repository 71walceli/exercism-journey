mod prine_number_generator;

use crate::prine_number_generator::{generate_primes, verify_divisibility};
use itertools::Itertools;

pub fn factors(number: u64) -> Vec<u64> {
    if (2u64..4u64).contains(&number) {
        return vec![number];
    }
    
    let primes = generate_primes(number);
    let mut factors: Vec<u64> = primes
        .into_iter().filter(|_prime| number.is_multiple_of(*_prime))
        .collect()
    ;
    
    factors.append(&mut factors.iter()
        .map(|_factor| number / _factor)
        .filter(|_factor| verify_divisibility(factors.iter(), *_factor))
        .collect()
    );
    
    factors.dedup();
    
    factors = factors.iter()
        .flat_map(|_factor| {
            let _factor = *_factor;
            let mut _number = number;
            let mut repeats = vec!();
            
            loop {
                if _number.is_multiple_of(_factor) {
                    _number /= _factor;
                    repeats.push(_factor);
                } else {
                    break;
                }
            }
            
            repeats
        })
        .collect()
    ;
    
    for k in 2..=factors.len() {
        factors.append(&mut factors.iter().combinations(k)
            .unique()
            .filter_map(|combination| {
                let _factor = combination.iter().map(|__factor| **__factor).product();
                let candidate = number / _factor;

                if number.is_multiple_of(_factor) 
                    && verify_divisibility(factors.iter(), candidate)
                    && _factor != number
                    && factors.binary_search(&candidate).is_err()
                {
                    Some(candidate)
                } else {
                    None
                }
            })
            .collect()
        )
    }

    factors
}
