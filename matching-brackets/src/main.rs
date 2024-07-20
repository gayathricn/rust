use matching_brackets::brackets_are_balanced;
fn main() {
    let string = "{[()]}";
    println!("The brackets are balanced: {}", brackets_are_balanced(string));
}