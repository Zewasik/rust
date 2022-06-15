#[derive(Debug)]
pub struct Numbers<'a> {
    pub numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        match self.numbers.last() {
            Some(value) => Some(*value),
            None => None,
        }
    }

    pub fn highest(&self) -> Option<u32> {
        let mut big = 0;
        if self.numbers.len() == 0 {
            return None;
        }
        for value in self.numbers {
            big = if *value > big { *value } else { big }
        }

        Some(big)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut ans = self.numbers.to_vec();

        ans.sort_by(|a, b| b.cmp(a));
        if ans.len() < 3 {
            ans
        } else {
            ans[0..3].to_vec()
        }
    }
}
