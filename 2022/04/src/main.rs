extern crate gcollections;
extern crate interval;

use crate::interval::interval_set::*;
use gcollections::ops::*;
use itertools::Itertools;

use std::io::{self, BufRead};

fn solve(input: impl BufRead) -> (i64, i64) {
    let lines = input.lines();
    let sets = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .map(|v| {
            v.split(',')
                .map(|p| {
                    p.split('-')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .collect::<Vec<Vec<(i64, i64)>>>();

    let part1 = sets
        .iter()
        .map(|v| {
            v.iter()
                .map(|t| vec![*t].to_interval_set())
                .collect_tuple()
                .unwrap()
        })
        .map(|(s1, s2)| (s1.clone(), s2.clone(), s1.join(s2)))
        .map(|(s1, s2, s)| if s == s1 || s == s2 { 1 } else { 0 })
        .sum();

    let part2 = sets
        .iter()
        .map(|v| {
            v.iter()
                .map(|t| vec![*t].to_interval_set())
                .collect_tuple()
                .unwrap()
        })
        .map(|(s1, s2)| {
            if s1.join(s2).interval_count() == 1 {
                1
            } else {
                0
            }
        })
        .sum();

    (part1, part2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (2, 4));
    }
}
