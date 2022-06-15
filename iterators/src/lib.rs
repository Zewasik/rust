pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(v: u64) -> Collatz {
        Collatz { v }
    }
}

impl Iterator for Collatz {
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        } else if self.v % 2 == 0 {
            self.v = self.v / 2
        } else {
            self.v = self.v * 3 + 1;
        };
        Some(self.v)
    }

    type Item = u64;
}

pub fn collatz(n: u64) -> Option<u64> {
    let mut i = 0;
    let mut temp = Collatz::new(n);

    while temp.next().is_some() {
        i += 1;
    }
    Some(i)
}
