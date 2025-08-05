pub struct Luhn {
    code: Vec<char>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let mut sum = 0;
        let code: Vec<char> = self.code.clone().into_iter().filter(|c| !c.is_whitespace()).collect();
    
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

impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        Self {
            code: input.chars().collect(),
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Self {
            code: input.chars().collect()
        }
    }
}

macro_rules! impl_from_unsigned_for_luhn {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Luhn {
                fn from(input: $t) -> Self {
                    Self {
                        code: input.to_string().chars().collect()
                    }
                }
            }
        )*
    };
}

impl_from_unsigned_for_luhn!(u8, u16, u32, u64, u128, usize);
