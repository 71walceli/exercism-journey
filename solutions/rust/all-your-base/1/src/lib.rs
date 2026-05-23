use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if let Some(digit) = number.iter().find(|digit| **digit >= from_base) {
        return Err(Error::InvalidDigit(*digit));
    }    
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    
    let mut value = BigUint::new(vec![0]);
   
    number.iter().enumerate().for_each(|(i, digit)| {
        value += *digit * from_base.pow((number.len() -i) as u32 -1u32);
    });
    
    let mut result = Vec::new();
    while value > BigUint::new(vec![0]) {
        result.push((value.clone() % to_base).to_u32_digits().first().map_or(0u32, |x| *x));
        value /= to_base;
    }
    result.reverse();
    
    Ok(if result.is_empty() { vec![0] } else { result })
}
