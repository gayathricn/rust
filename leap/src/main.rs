use leap::is_leap_year;
fn main(){
    let year=0;
    if is_leap_year(year){
        println!("The year {} is a leap year",year);
    }else{
        println!("The year {} is not a leap year",year,);
    }
}