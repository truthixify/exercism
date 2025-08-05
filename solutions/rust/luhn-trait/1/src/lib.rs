use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let mut sum = 0;
        let code: Vec<char> = self.to_string().chars().filter(|c| !c.is_whitespace()).collect();
    
        if code.len() <= 1 {
            return false;
        }
    
        for (i, ch) in code.iter().rev().enumerate() {
            if !ch.is_ascii_digit() {
                return false;
            }
            
            let parsed_num = ch.to_digit(10).unwrap();
            if i % 2 == 1 {
                if parsed_num * 2 > 9 {
                    sum += 2 * parsed_num - 9;
                } else {
                   sum += 2 * parsed_num;
                }
            } else {
                sum += parsed_num;
            }
        }
    
        sum.is_multiple_of(10)
    }
}