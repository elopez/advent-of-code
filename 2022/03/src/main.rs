#![feature(exclusive_range_pattern)]
#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::io::{self, BufRead};
use std::ops::BitAnd;

fn convert_priority(val: &u8) -> i32 {
    match val {
        65..91 => (val - 65 + 27).into(),
        97..123 => (val - 96).into(),
        _ => panic!("unknown {val}"),
    }
}

fn solve(input: impl BufRead) -> (i32, i32) {
    let lines = input.lines();
    let bags = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap().as_bytes().to_vec())
            }
        })
        .map(|v| v.iter().map(convert_priority).collect())
        .collect::<Vec<Vec<i32>>>();

    let result_p1: i32 = bags
        .iter()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(x, y)| -> (HashSet<i32>, HashSet<i32>) {
            (
                HashSet::from_iter(x.to_vec()),
                HashSet::from_iter(y.to_vec()),
            )
        })
        .map(|(x, y)| x.bitand(&y).iter().sum::<i32>())
        .sum();

    let result_p2: i32 = bags
        .iter()
        .map(|x| -> HashSet<i32> { HashSet::from_iter(x.to_vec()) })
        .array_chunks::<3>()
        .map(|[x, y, z]| x.bitand(&y).bitand(&z).iter().sum::<i32>())
        .sum();

    (result_p1, result_p2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (157, 70));
    }
}
