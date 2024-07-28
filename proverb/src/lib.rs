pub fn build_proverb(list: &[&str]) -> String {
    //todo!("build a proverb from this list of items: {list:?}")
    let mut result = String::new();
    if list.is_empty(){
        return String::new();
    }
    for i in 0..list.len()-1 {
        result.push_str(&format!("For want of a {} the {} was lost.\n",list[i],list[i+1]));
    }
    if !list.is_empty() {
        result.push_str(&format!("And all for the want of a {}.", list[0]));
    }
result
}
