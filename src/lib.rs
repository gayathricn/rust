pub fn reverse(input: &str) -> String {
    //todo!("Write a function to reverse {input}");
    return input.chars().rev().collect();
}
fn main(){
    let st = String::new();
    println!("Reverse of the string is {}", reverse(&st));
}
