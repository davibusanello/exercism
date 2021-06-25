pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    };
    let number_as_string = num.to_string();
    let mut sum: u32 = 0;

    for digit_char in number_as_string.chars() {
        sum += digit_char
            .to_digit(10)
            .unwrap()
            .pow(number_as_string.len() as u32);
    }

    num == sum as u32
}
