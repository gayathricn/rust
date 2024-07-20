use std::io::empty;
pub fn brackets_are_balanced(string: &str) -> bool {
//todo!("Check if the string \"{string}\" contains balanced brackets");
    let mut result = Vec::new();
    for item in string.chars() {
        match item {
            '{' | '[' | '(' => result.push(item),
            '}' => {
                if result.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if result.pop() != Some('[') {
                    return false;
                }
            }
            ')' => {
                if result.pop() != Some('(') {
                    return false;
                }
            }
            _ => {}
        }
    }
result.is_empty()
}
