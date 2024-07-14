
use armstrong_numbers::is_armstrong_number;
fn main(){
    //let mut number = u32::new();
    let input:u32= 0;
    if is_armstrong_number(input){
        println!("The number {} is amstrong", input);
    }else{
        println!("The number {} is not amstrong", input);
    }
}