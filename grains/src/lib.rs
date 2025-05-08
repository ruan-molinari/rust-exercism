pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!()
    }

    (1..s).fold(1, |ac: u64, _| ac * 2)
}

pub fn total() -> u64 {
    (1..65)
        .reduce(|ac: u64, x| ac + square(x.try_into().unwrap()))
        .unwrap()
}
