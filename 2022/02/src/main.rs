#![feature(is_some_and)]

use std::io::{self, BufRead};

#[derive(Copy, Clone)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

#[derive(Copy, Clone)]
enum Result {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

struct Match {
    //them: Move,
    us: Move,
    result: Result,
}

impl Match {
    fn from_string_part_1(s: &str) -> Self {
        let mut iter = s.split(" ");
        let them = convert_input_part_1(iter.next().unwrap());
        let us = convert_input_part_1(iter.next().unwrap());
        let result = match_result(them, us);
        Match { us, result }
    }

    fn from_string_part_2(s: &str) -> Self {
        let mut iter = s.split(" ");
        let them = convert_input_part_1(iter.next().unwrap());
        let result = convert_input_part_2(iter.next().unwrap());
        let us = match_play(them, result);
        Match { us, result }
    }

    fn score(self) -> i64 {
        self.us as i64 + self.result as i64
    }
}

fn convert_input_part_1(item: &str) -> Move {
    match item {
        "A" => Move::ROCK,
        "X" => Move::ROCK,
        "B" => Move::PAPER,
        "Y" => Move::PAPER,
        "C" => Move::SCISSORS,
        "Z" => Move::SCISSORS,
        _ => panic!("unknown"),
    }
}

fn convert_input_part_2(item: &str) -> Result {
    match item {
        "X" => Result::LOSE,
        "Y" => Result::DRAW,
        "Z" => Result::WIN,
        _ => panic!("unknown"),
    }
}

fn match_result(them: Move, us: Move) -> Result {
    match them {
        Move::ROCK => match us {
            Move::ROCK => Result::DRAW,
            Move::PAPER => Result::WIN,
            Move::SCISSORS => Result::LOSE,
        },
        Move::PAPER => match us {
            Move::ROCK => Result::LOSE,
            Move::PAPER => Result::DRAW,
            Move::SCISSORS => Result::WIN,
        },
        Move::SCISSORS => match us {
            Move::ROCK => Result::WIN,
            Move::PAPER => Result::LOSE,
            Move::SCISSORS => Result::DRAW,
        },
    }
}

fn match_play(them: Move, result: Result) -> Move {
    match them {
        Move::ROCK => match result {
            Result::DRAW => Move::ROCK,
            Result::WIN => Move::PAPER,
            Result::LOSE => Move::SCISSORS,
        },
        Move::PAPER => match result {
            Result::LOSE => Move::ROCK,
            Result::DRAW => Move::PAPER,
            Result::WIN => Move::SCISSORS,
        },
        Move::SCISSORS => match result {
            Result::WIN => Move::ROCK,
            Result::LOSE => Move::PAPER,
            Result::DRAW => Move::SCISSORS,
        },
    }
}

fn solve(input: impl BufRead) -> (i64, i64) {
    let lines = input.lines();
    let moves = lines
        .map_while(|p| {
            if p.as_ref().unwrap().is_empty() {
                None
            } else {
                Some(p.unwrap())
            }
        })
        .collect::<Vec<String>>();
    let score_p1 = moves
        .iter()
        .map(|p| Match::from_string_part_1(&p))
        .map(|m| m.score())
        .sum();
    let score_p2 = moves
        .iter()
        .map(|p| Match::from_string_part_2(&p))
        .map(|m| m.score())
        .sum();

    (score_p1, score_p2)
}

fn main() {
    let (score_p1, score_p2) = solve(io::stdin().lock());
    println!("Total score: {score_p1} / {score_p2}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_case() {
        let case = "A Y
B X
C Z";
        let result = super::solve(case.as_bytes());
        assert_eq!(result, (15, 12));
    }
}
