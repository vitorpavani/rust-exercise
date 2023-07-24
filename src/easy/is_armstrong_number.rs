pub fn is_armstrong_number(num: u64) -> bool {
    let mut digits: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 0 {
        digits.push((n % 10).try_into().unwrap());
        n /= 10;
    }
    let len: u64 = digits.len() as u64;
    let sum = digits
        .iter()
        .fold(0, |acc, x| acc + x.pow(len.try_into().unwrap()));
    if sum != num {
        return false;
    }
    return true;
}
