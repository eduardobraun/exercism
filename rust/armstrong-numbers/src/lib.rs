pub fn is_armstrong_number(num: u32) -> bool {
    let decimal = num.to_string();
    let p = decimal.len() as u32;
    let sum = decimal
        .chars()
        .fold(0u32, |v, c| c.to_digit(10).unwrap().pow(p) + v);
    sum == num
}
