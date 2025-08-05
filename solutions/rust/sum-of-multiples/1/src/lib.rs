use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();

    for factor in factors {
        if *factor == 0 {
            continue;
        }
        
        for i in 1..=(limit / factor) {
            let current_multiple = factor * i;
            if current_multiple < limit {
                set.insert(factor * i);
            }
        }
    }

    set.iter().sum()
}