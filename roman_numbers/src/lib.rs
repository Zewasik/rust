#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(v: u32) -> RomanDigit {
        match v {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut v: u32) -> RomanNumber {
        if v == 0 {
            return RomanNumber(vec![RomanDigit::from(0)]);
        }
        let mut ans: Vec<RomanDigit> = vec![];
        let mut rank = 10_u32.pow(v.to_string().chars().count() as u32 - 1);

        while v > 0 {
            let current_digit = v / rank;
            match current_digit {
                1..=3 => {
                    for _ in 1..=current_digit {
                        ans.push(RomanDigit::from(rank))
                    }
                }
                4 => {
                    ans.push(RomanDigit::from(rank));
                    ans.push(RomanDigit::from(rank * 5))
                }
                5..=8 => {
                    ans.push(RomanDigit::from(rank * 5));
                    for _ in 1..=current_digit - 5 {
                        ans.push(RomanDigit::from(rank))
                    }
                }
                9 => {
                    ans.push(RomanDigit::from(rank));
                    ans.push(RomanDigit::from(rank * 10))
                }
                _ => (),
            }

            v %= rank;
            rank /= 10;
        }

        RomanNumber(ans)
    }
}
