use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct AlphameticAddition {
    addends: Vec<String>,
    result: String,
}

//impl Copy for AlphameticAddition {}

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

    Ok(AlphameticAddition {
        addends: addends.split("+").map(|e| String::from(e.trim())).collect(),
        result: String::from(result.trim()),
    })
}

fn map_characters(addition: &AlphameticAddition, values: &HashMap<char, u8>) -> AlphameticAddition {
    AlphameticAddition {
        addends: addition.addends
            .iter()
            .cloned()
            .map(|e| values.iter()
                .fold(e, |res, (k, v)| res.replace(*k, v.to_string().as_str()))
            )
            .collect(),
        result: values.iter()
            .fold(
                addition.result.to_owned(), 
                |e2, (k, v)| e2.replace(*k, v.to_string().as_str())
            )
        ,
    }
}

fn evaluate(addition: &AlphameticAddition) -> Result<bool, &'static str> {
    if addition.addends.iter().map(|e| e.parse::<u64>()).find(|e| e.is_err()).is_some() {
        return Err("Some terms still have undecoded letters.");
    }
    if addition.result.parse::<u64>().is_err() {
        return Err("Result still has letters.")
    }
    
    Ok(
        addition.addends.iter().map(|e| e.parse::<u64>())
            .fold(0, |total, e| total + e.unwrap())
        == addition.result.parse::<u64>().unwrap()
    )
}

pub fn _solve(input: &str) -> Result<HashMap<char, u8>, &'static str> {
    let addition = normalize(input)?;

    let keys: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).unique().collect();
    let values_perm = (0..10).permutations(keys.len());
    
    //let mut values = HashMap::new();
    let values = values_perm
        .map(|perm| {
            let mut values = HashMap::new();
            keys.iter().zip(perm.iter()).for_each(|(k, v)| {
                values.insert(*k, *v);
            });
            
            (map_characters(&addition, &values), values)
        })
        .filter(|(e, _)|
            e.addends.iter().all(|c| !c.starts_with('0')) && !e.result.starts_with('0')
        )
        .find(|(e, _)| {
            let result = evaluate(e);
            match result {
                Ok(v) => v,
                Err(r) => {
                    println!("Error: {r}");
                    false
                },
            }
        })
    ;
    
    if let Some(values) = values {
        Ok(values.1)
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
