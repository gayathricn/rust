use diffie_hellman::secret;
fn main(){
    let p: u64 = 23;   
    let b_pub: u64 = 5;
    let a: u64 = 6;
    println!("The secret is: {}", secret(p, b_pub, a));
}