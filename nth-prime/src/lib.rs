pub fn nth(n: u32) -> u32 {
    //todo!("What is the 0-indexed {n}th prime number?")
    if n == 0 {
        return 2;
    }else
    {
        let mut count  = 0;
        let mut num = 1;
        while count <= n {
            num += 1;
            if is_prime(num) {
                count += 1;
            }
        }
        num
    }
}
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    let limit = (num as f64).sqrt() as u32;
    for i in 2 ..=limit {
        if num%i == 0 {
            return false;
        }
    }
    return true
}
