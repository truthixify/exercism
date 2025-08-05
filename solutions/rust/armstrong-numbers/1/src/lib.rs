pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 || num.ilog10() == 0 {
        return true;
    }
    
    let num_of_digits = num.ilog10() + 1;
    let mut rad = 1;
    let mut sum = 0;

    for _ in 0..num_of_digits {
        let current_number: u32 = (num % (rad * 10) - num % rad) / rad;
        
        sum += current_number.pow(num_of_digits);
        rad *= 10;
    }

    num == sum
}
