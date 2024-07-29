pub fn collatz(n: u64) -> Option<u64> {
    //todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut count:u64 = 0;
    let mut num = n;
    if num == 0{
        return None;
    }
    while num>1{
        if num%2 == 0{
            num = num/2;
        }else{
            num = 3*num +1;
        }
        count += 1
    }
    Some(count)
}
