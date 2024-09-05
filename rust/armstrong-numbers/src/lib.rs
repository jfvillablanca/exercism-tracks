pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let num_chars = num_string.chars();
    let num_len = num_chars.clone().collect::<Vec<_>>().len() as u32;
    num == num_chars.map(|c| {
        c.to_digit(10).unwrap().pow(num_len)
    }).sum::<u32>()

    // todo!("true if {num} is an armstrong number")
}
