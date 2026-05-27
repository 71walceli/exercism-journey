use itertools::Itertools;
use std::{collections::{BTreeSet, HashMap}, iter};

#[derive(Clone, Debug)]
struct AlphameticAddition {
    addends: Vec<Vec<u8>>,
    result: Vec<u8>,
}

fn normalize(input: &str) -> Result<AlphameticAddition, &'static str> {
    if input.chars()
        .any(|c| !c.is_alphabetic() && !"+=".chars().any(|_c| _c == c) && !c.is_whitespace())
    {
        return Err("Invalid characters");
    }
    if input.chars().filter(|c| c.is_alphabetic()).unique().count() > 10 {
        return Err("Too many characters");
    }

    let input = input.to_ascii_uppercase();
    let (addends, result) = input.split("==").next_tuple().ok_or("Missing ==")?;

    if addends.split("+").count() < 2 {
        return Err("Not enough addends");
    }
    
    let addends: Vec<String> = addends.split("+").map(|e| String::from(e.trim())).collect();
    let result = String::from(result.trim());
    let max_length = addends.iter().map(|e| e.len()).chain(iter::once(result.len())).max().unwrap();
    let addends = addends.iter().map(
        |e| format!("{:>width$}", e, width = max_length).as_bytes().to_vec()
    ).collect();
    let result = format!("{:>width$}", result, width = max_length)
        .as_bytes().to_vec()
    ;

    Ok(AlphameticAddition { addends, result, })
}

/**
 Returns a result. If no error, the OK value is a tuple.
 * The OK value is a tuple consisting of a Boolean and a carry.
   * The boolean is whether it is some odds up or not at expected value.
   * The carry consists of any digits that go past the unit for this row. 
 * In case of error, it returns a string slice.
 */
fn evaluate_column(row: &[u8], values: &HashMap<char, u8>, carry: u32) 
-> Result<(bool, u32), &'static str> {
    let row: Result<Vec<u8>, &'static str> = row.iter().copied()
        .map(|e| values.get(&(e as char))
            .map_or(if e as char == ' ' { 0 } else { e }, |x| *x))
        .map(|e| if e > 9 { Err("Unexpected non-digit") } else { Ok(e) })
        .collect()
    ;
    let row = row?;
    
    let addends = &row[0..row.len()-1];
    let result = row[row.len()-1];
    
    let sum = addends.iter().map(|e| *e as u32).sum::<u32>() + carry;
    Ok(( (sum % 10) as u8 == result, sum / 10) )
}

pub fn _solve(input: &str) -> Result<HashMap<char, u8>, &'static str> {
    let addition = normalize(input)?;

    let keys: BTreeSet<char> = input.chars().filter(|c| c.is_alphabetic()).unique().collect();
    let values_perm = (0u8..10u8).permutations(keys.len());
    
    let values = values_perm
        .map(|perm| {
            let mut values = HashMap::new();
            keys.iter().zip(perm.iter()).for_each(|(k, v)| {
                values.insert(*k, *v);
            });
            
            values
        })
        .filter(|values| addition.addends.iter()
            .chain(iter::once(&addition.result))
            .all(|e| e.iter()
                .find(|c| **c as char != ' ')
                .map(|c| *values.get(&(*c as char)).unwrap()).unwrap() != 0u8
            ))
        .find(|values| {
            let mut carry = 0;
            addition.result.iter().rev().enumerate().all(|(i, r)| {
                let column = addition.addends.iter()
                    .map(|e| e[e.len() -i -1])
                    .chain(iter::once(*r))
                    .collect::<Vec<u8>>()
                ;
                
                let result = evaluate_column(&column, values, carry);
                if result.is_err() {
                    println!("Error: {}", result.err().unwrap());
                    false
                } else {
                    let result = result.ok().unwrap();
                    carry = result.1;
                    
                    result.0
                }
            })
        })
    ;
    
    if let Some(values) = values {
        Ok(values)
    } else {
        Err("No solution found")
    }
}   

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let result = _solve(input);
    if result.is_err() {
        println!("Error: {}", result.as_ref().err().unwrap());
    }
    result.ok()
}
