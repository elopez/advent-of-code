extern crate nom;
use nom::sequence::terminated;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::{take, take_until},
    character::complete::*,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Sum(String, String),
    Sub(String, String),
    Times(String, String),
    Div(String, String),
    Const(isize),
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    name: String,
    operation: Operation,
}

fn parse_op(input: &str) -> IResult<&str, Operation> {
    let (input, fst) = terminated(take_until(" "), tag(" "))(input)?;
    let (input, sign) =
        terminated(alt((char('*'), char('+'), char('-'), char('/'))), tag(" "))(input)?;
    let (input, snd) = take(4usize)(input)?;

    Ok((
        input,
        match sign {
            '+' => Operation::Sum(fst.to_string(), snd.to_string()),
            '-' => Operation::Sub(fst.to_string(), snd.to_string()),
            '*' => Operation::Times(fst.to_string(), snd.to_string()),
            '/' => Operation::Div(fst.to_string(), snd.to_string()),
            _ => panic!("unknown"),
        },
    ))
}

fn parse_number(input: &str) -> IResult<&str, Operation> {
    let (input, num) = map_res(digit1, isize::from_str)(input)?;
    Ok((input, Operation::Const(num)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, name) = terminated(take_until(":"), tag(": "))(input)?;

    let (input, op) = alt((parse_op, parse_number))(input)?;

    Ok((
        input,
        Monkey {
            name: name.to_string(),
            operation: op,
        },
    ))
}

fn all_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n"), parse_monkey)(input)
}

fn dfs_sum(tree: &HashMap<String, (Option<bool>, Operation)>, key: &str) -> isize {
    match &tree[key].1 {
        Operation::Sum(a, b) => dfs_sum(tree, a) + dfs_sum(tree, b),
        Operation::Sub(a, b) => dfs_sum(tree, a) - dfs_sum(tree, b),
        Operation::Times(a, b) => dfs_sum(tree, a) * dfs_sum(tree, b),
        Operation::Div(a, b) => dfs_sum(tree, a) / dfs_sum(tree, b),
        Operation::Const(i) => *i,
    }
}

fn dfs_human_taint(tree: &mut HashMap<String, (Option<bool>, Operation)>, key: &str) -> bool {
    if key == "humn" {
        true
    } else if let Some(x) = &tree[key].0 {
        *x
    } else {
        let res = match &tree[key].1.clone() {
            Operation::Sum(a, b)
            | Operation::Sub(a, b)
            | Operation::Times(a, b)
            | Operation::Div(a, b) => dfs_human_taint(tree, a) || dfs_human_taint(tree, b),
            Operation::Const(_) => false,
        };
        tree.insert(key.to_string(), (Some(res), tree[key].1.clone()));
        res
    }
}

fn dfs_recover(
    tree: &mut HashMap<String, (Option<bool>, Operation)>,
    key: &str,
    number: isize,
) -> isize {
    if key == "humn" {
        tree.insert(key.to_string(), (Some(true), Operation::Const(number)));
    }
    match &tree[key].1.clone() {
        Operation::Sum(a, b) => {
            if dfs_human_taint(tree, a) {
                dfs_recover(tree, a, number - dfs_sum(tree, b))
            } else {
                dfs_recover(tree, b, number - dfs_sum(tree, a))
            }
        }
        Operation::Sub(a, b) => {
            if dfs_human_taint(tree, a) {
                dfs_recover(tree, a, number + dfs_sum(tree, b))
            } else {
                dfs_recover(tree, b, dfs_sum(tree, a) - number)
            }
        }
        Operation::Times(a, b) => {
            if dfs_human_taint(tree, a) {
                dfs_recover(tree, a, number / dfs_sum(tree, b))
            } else {
                dfs_recover(tree, b, number / dfs_sum(tree, a))
            }
        }
        Operation::Div(a, b) => {
            if dfs_human_taint(tree, a) {
                dfs_recover(tree, a, number * dfs_sum(tree, b))
            } else {
                dfs_recover(tree, b, dfs_sum(tree, a) / number)
            }
        }
        Operation::Const(i) => *i,
    }
}

fn solve_p1(monkeys: &[Monkey]) -> isize {
    let mut tree: HashMap<String, (Option<bool>, Operation)> = HashMap::new();
    for m in monkeys.iter().cloned() {
        tree.insert(m.name, (None, m.operation));
    }

    dfs_sum(&tree, "root")
}

fn solve_p2(monkeys: &[Monkey]) -> isize {
    let mut tree: HashMap<String, (Option<bool>, Operation)> = HashMap::new();
    for m in monkeys.iter().cloned() {
        tree.insert(m.name, (None, m.operation));
    }

    let (m1, m2) = match &tree["root"].1 {
        Operation::Sum(a, b)
        | Operation::Sub(a, b)
        | Operation::Times(a, b)
        | Operation::Div(a, b) => (a.to_string(), b.to_string()),
        Operation::Const(_) => panic!("root is not op"),
    };

    let (good, bad) = if dfs_human_taint(&mut tree, &m1) {
        (m2, m1)
    } else {
        (m1, m2)
    };
    let sum = dfs_sum(&tree, &good);

    dfs_recover(&mut tree, &bad, sum);

    if let Operation::Const(x) = tree["humn"].1 {
        x
    } else {
        panic!("oops")
    }
}

fn solve(mut input: impl BufRead) -> (isize, isize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, monkeys) = all_input(&buffer).unwrap();

    let p1 = solve_p1(&monkeys);
    let p2 = solve_p2(&monkeys);

    (p1, p2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
        assert_eq!(
            parse_monkey("root: pppw + sjmn"),
            Ok((
                "",
                Monkey {
                    name: "root".to_string(),
                    operation: Operation::Sum("pppw".to_string(), "sjmn".to_string())
                }
            ))
        );
    }

    #[test]
    fn test_sample() {
        let sample = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(solve(sample.as_bytes()), (152, 301));
    }
}
