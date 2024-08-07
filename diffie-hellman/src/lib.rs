use rand::Rng;
pub fn private_key(p: u64) -> u64 {
    //todo!("Pick a private key greater than 1 and less than {p}")
    if p <= 2 {
        panic!("The value of p must be greater than 2");
    }
    
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if (exponent % 2) == 1 {  
            result = (result * base) % modulus;
        }
        exponent >>= 1;  
        base = (base * base) % modulus;  
    }
    result
}


pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    //todo!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    //todo!("Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}")
    modular_exponentiation(b_pub, a, p)
}