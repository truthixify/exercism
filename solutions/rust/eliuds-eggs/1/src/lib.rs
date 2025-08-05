pub fn egg_count(display_value: u32) -> usize {
    let num_len = format!("{display_value:b}").len();
    let mut count = 0;

    for i in 0..num_len {
        if (display_value >> i) & 1 == 1 {
            count += 1;
        }
    }

    count
}
