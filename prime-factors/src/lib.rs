pub fn factors(n: u64) -> Vec<u64> {
    //todo!("This should calculate the prime factors of {n}")
    fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                return false;
            }
        }   
        true
    }

    let primes: Vec<u64> = (2..=((n as f64).sqrt() as u64)).filter(|&x| is_prime(x)).collect();

    let mut factors = Vec::new();
    let mut rem = n;

    for &prime in &primes{
        while rem%prime == 0{
            factors.push(prime);
            rem/=prime;
        }
    }
    if rem > 1 {
        factors.push(rem);
    }
    factors.sort();
    factors
}
