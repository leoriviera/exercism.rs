pub fn is_armstrong_number(num: u32) -> bool {
    let as_string = num.to_string();
    let chars = as_string.chars();
    let length = as_string.len().try_into().unwrap();

    let power_sum: u128 = chars
        .map(|n| n.to_digit(10).unwrap().pow(length) as u128)
        .sum::<u128>();

    power_sum == num as u128
}
