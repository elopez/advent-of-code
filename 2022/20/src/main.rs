extern crate nom;
use nom::{
    branch::alt,
    character::complete::*,
    combinator::{map_res, opt},
    multi::separated_list1,
    IResult,
};
use std::collections::VecDeque;
use std::io::{self, Read};
use std::str::FromStr;

fn parse_number(input: &str) -> IResult<&str, isize> {
    let (input, opt_sign) = opt(alt((char('+'), char('-'))))(input)?;
    let sign = match opt_sign {
        Some('+') => 1,
        Some('-') => -1,
        _ => 1,
    };
    let (input, nr) = map_res(digit1, isize::from_str)(input)?;
    Ok((input, nr * sign))
}

fn all_input(input: &str) -> IResult<&str, Vec<isize>> {
    separated_list1(newline, parse_number)(input)
}

fn positive_modulo(nr: usize, x: isize, modulo: usize) -> usize {
    (((nr as isize + x) % modulo as isize) + modulo as isize) as usize % modulo
}

fn solve(buffer: &str, key: isize, loops: usize) -> isize {
    let (_, list) = all_input(buffer).unwrap();

    let modulo = list.len();
    let mut modified = VecDeque::from_iter(list.into_iter().enumerate().map(|(i, e)| (e * key, i)));
    let mut i = 0;
    let mut count = 0;

    while count < modulo * loops {
        while modified[positive_modulo(i, 0, modulo)].1 != count % modulo {
            i += 1;
        }
        let rm_index = positive_modulo(i, 0, modulo);
        let rm = modified.remove(rm_index).unwrap();
        let insert_index = positive_modulo(rm_index, rm.0, modulo - 1);

        modified.insert(insert_index, rm);

        count += 1;
    }

    let zeropos = modified
        .iter()
        .enumerate()
        .filter_map(|(i, (nr, _))| if *nr == 0 { Some(i) } else { None })
        .sum();

    [1000, 2000, 3000]
        .map(|n| modified[positive_modulo(zeropos, n, modulo)].0)
        .into_iter()
        .sum()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let p1 = solve(&buffer, 1, 1);
    println!("solve 2!");
    let p2 = solve(&buffer, 811589153, 10);
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let sample = "1
2
-3
3
-2
0
4";

        assert_eq!(all_input(sample), Ok(("", vec![1, 2, -3, 3, -2, 0, 4])))
    }

    #[test]
    fn test_sample() {
        let sample = "1
2
-3
3
-2
0
4";

        assert_eq!(solve(sample, 1, 1), 3);
        assert_eq!(solve(sample, 811589153, 10), 1623178306)
    }
}
