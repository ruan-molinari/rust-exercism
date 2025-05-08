pub fn is_armstrong_number(num: u32) -> bool {
    let len: u32 = num.to_string().len().try_into().unwrap();

    let sum = num
        .to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .fold(0, |ac, x| ac + x.pow(len));

    num == sum
}
