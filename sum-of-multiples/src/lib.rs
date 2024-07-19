pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
let mut result = Vec::new();
for &item in factors{
    if item>0
    {
        for multiple in (item..limit).step_by(item as usize) {
            result.push(multiple); 
        }
    }
    
}
result.sort();
result.dedup();
result.iter().sum()
}
