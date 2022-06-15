pub struct Collatz {
    pub v: u64,
}

// impl Iterator for Collatz {}

pub fn collatz(n: u64) -> Option<u64> {
    let mut i = 0;
    let mut v = n;

    while v != 1 {
        i += 1;
        v = if v % 2 == 0 { v / 2 } else { v * 3 + 1 };
    }
    Some(i)
}
