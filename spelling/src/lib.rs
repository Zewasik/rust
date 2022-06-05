use std::collections::HashMap;

pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut split_thousand: Vec<u64> = Vec::new();
    let mut ans = String::new();

    {
        let mut tempn = n;
        while tempn > 0 {
            split_thousand.push(tempn % 1000);
            tempn /= 1000;
        }
    }

    for (index, value) in split_thousand.iter().enumerate().rev() {
        ans += &triplet_convert(value);

        match index {
            2 => ans += " million ",
            1 => ans += " thousand ",
            _ => (),
        };
    }

    ans
}

fn triplet_convert(num: &u64) -> String {
    let tens = HashMap::from([
        (2, "twenty"),
        (3, "thirty"),
        (4, "fourty"),
        (5, "fifty"),
        (6, "sixty"),
        (7, "seventy"),
        (8, "eighty"),
        (9, "ninety"),
    ]);

    let units = HashMap::from([
        (0, ""),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ]);

    let uncommon = HashMap::from([
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
    ]);

    let mut ans = String::new();
    let length = num.to_string().chars().count();

    match length {
        1 => ans += units.get(num).unwrap(),
        2 => {
            if *num < 20 {
                ans += uncommon.get(num).unwrap()
            } else {
                ans = ans + tens.get(&(num / 10)).unwrap() + "-" + units.get(&(num % 10)).unwrap()
            }
        }
        _ => {
            ans = ans
                + units.get(&(num / 100)).unwrap()
                + " hundred "
                + &triplet_convert(&(num % 100))
        }
    };

    ans
}

// nine hundred ninety-nine thousand nine hundred ninety-nine
