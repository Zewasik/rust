pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
    pub now: T,
}

use std::ops::Add;
impl<T> StepIterator<T>
where
    T: Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            beg,
            end,
            step,
            now: beg,
        }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.now <= self.end {
            let now = self.now;
            self.now = self.now + self.step;
            Some(now)
        } else {
            None
        }
    }
}
