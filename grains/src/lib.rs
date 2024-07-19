pub fn square(s: u32) -> u64 {
    //todo!("grains of rice on square {s}");
    if s<1 || s>64 {
        panic!("Square must be between 1 and 64");
    }else {
        return 2_u64.pow(s-1);
    }
    }

pub fn total() -> u64 {
    //todo!();
    return (1..=64).map(square).sum()
}
