use core::fmt;
use std::{
    cmp::max,
    fmt::{Display, Formatter},
    io::{self, BufRead},
    iter::Sum,
    str::FromStr,
};

#[derive(Debug)]
struct BalancedQuinary(Vec<i8>);

#[derive(Debug, PartialEq, Eq)]
struct ParseBalancedQuinaryError;

impl FromStr for BalancedQuinary {
    type Err = ParseBalancedQuinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s
            .as_bytes()
            .iter()
            .map(|c| match *c {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => panic!("unknown"),
            })
            .collect();

        Ok(BalancedQuinary(number))
    }
}

impl Display for BalancedQuinary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for n in self.0.iter() {
            write!(
                f,
                "{}",
                match *n {
                    -2 => "=",
                    -1 => "-",
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    _ => "",
                }
            )?;
        }
        Ok(())
    }
}

impl Sum for BalancedQuinary {
    fn sum<I: ::core::iter::Iterator<Item = Self>>(iter: I) -> Self {
        let mut maxlen = 0;
        let nums: Vec<Self> = iter.collect();
        for n in nums.iter() {
            maxlen = max(maxlen, n.0.len())
        }

        let mut res = vec![0; maxlen];
        for bq in nums {
            let pos = maxlen - bq.0.len();
            for (i, d) in bq.0.iter().enumerate() {
                res[i + pos] += *d as isize
            }
        }

        BalancedQuinary::from_unbalanced(&res)
    }
}

impl BalancedQuinary {
    fn from_unbalanced(n: &[isize]) -> Self {
        let mut numbers = n.iter().rev();
        let mut res = vec![];
        let mut carry = 0;

        loop {
            let mut num = carry;
            if let Some(d) = numbers.next() {
                num += d;
            } else if num == 0 {
                break;
            }

            if num > 2 {
                let trim_times = (num + 2) / 5;
                num -= trim_times * 5;
                carry = trim_times;
            } else if num < -2 {
                let trim_times = (num - 2) / 5;
                num -= trim_times * 5;
                carry = trim_times;
            } else {
                carry = 0;
            }
            res.push(num as i8);
        }

        res.reverse();
        BalancedQuinary(res)
    }
}

fn solve(input: impl BufRead) -> String {
    let lines = input.lines();
    let numbers = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .map(|s| BalancedQuinary::from_str(&s).unwrap())
        .collect::<Vec<_>>();

    let part1 = numbers.into_iter().sum::<BalancedQuinary>();

    part1.to_string()
}

fn main() {
    let score_p1 = solve(io::stdin().lock());
    println!("Total score: {score_p1}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, "2=-1=0".to_string());
    }
}
