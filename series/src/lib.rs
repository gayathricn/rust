pub fn series(digits: &str, len: usize) -> Vec<String> {
    //todo!("What are the series of length {len} in string {digits:?}")
    let mut result = Vec::new();
    if len == 0 || len> digits.len(){
        return result;
    }
    for i in 0..=(digits.len()-len){
        result.push(digits[i..i+len].to_string());
        }
        result
}
