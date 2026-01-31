use itertools::Itertools;

pub fn sum_of_multiples(level: u32, base_values: &[u32]) -> u32 {
    base_values.iter()
        .filter(|base_value| **base_value != 0)
        .flat_map(|base_value| (1..).map(|value| value * base_value)
            .take_while(|value| *value < level)
            .collect::<Vec<u32>>()
        )
        .unique()
        .sum()
}
