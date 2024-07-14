pub fn is_armstrong_number(num: u32) -> bool {
    //todo!("true if {num} is an armstrong number")
    let mut n = num;
    let len = n.to_string().len() as u32;
    let mut sum:u64 = 0;
    while n != 0{
        let remainder= n%10;
    sum = sum + (remainder as u64).pow(len);
    n = n / 10;
    }
    return sum == num as u64;
}