extern crate nom;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::*, combinator::map_res,
    multi::separated_list1, IResult,
};
use num_integer::Integer;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Sum(usize),
    Times(usize),
    TimesSelf,
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    monkey_true: usize,
    monkey_false: usize,
    plays: usize,
}

impl Monkey {
    fn play_top(&mut self, relief: Option<usize>) -> Option<(usize, usize)> {
        if self.items.is_empty() {
            return None;
        }

        self.plays += 1;

        let item = self.items.remove(0);
        let inspect_worry = match self.operation {
            Operation::Sum(x) => item + x,
            Operation::Times(x) => item * x,
            Operation::TimesSelf => item.pow(2),
        };
        let relief_worry = if let Some(x) = relief {
            inspect_worry % x
        } else {
            inspect_worry / 3
        };

        if relief_worry.is_multiple_of(&self.test) {
            Some((self.monkey_true, relief_worry))
        } else {
            Some((self.monkey_false, relief_worry))
        }
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _monkey_nr) = digit1(input)?;

    let (input, _) = tag(":\n  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), map_res(digit1, usize::from_str))(input)?;
    let (input, _) = tag("\n  Operation: new = old ")(input)?;
    let (input, sign) = alt((char('*'), char('+')))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, oper) = alt((tag("old"), digit1))(input)?;
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, test) = map_res(digit1, usize::from_str)(input)?;
    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, monkey_true) = map_res(digit1, usize::from_str)(input)?;
    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, monkey_false) = map_res(digit1, usize::from_str)(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation: match sign {
                '+' => Operation::Sum(oper.parse().unwrap()),
                '*' => match oper {
                    "old" => Operation::TimesSelf,
                    _ => Operation::Times(oper.parse().unwrap()),
                },
                _ => panic!("unknown"),
            },
            test,
            monkey_true,
            monkey_false,
            plays: 0,
        },
    ))
}

fn all_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(tag("\n\n"), parse_monkey)(input)
}

fn solve_p1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while let Some((x, item)) = monkeys[m].play_top(None) {
                monkeys[x].add_item(item);
            }
        }
    }

    monkeys.sort_by_cached_key(|m| m.plays);

    monkeys[monkeys.len() - 1].plays * monkeys[monkeys.len() - 2].plays
}

fn solve_p2(mut monkeys: Vec<Monkey>) -> usize {
    let mut lcm = 1;
    for m in &monkeys {
        lcm = lcm.lcm(&m.test);
    }

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            while let Some((x, item)) = monkeys[m].play_top(Some(lcm)) {
                monkeys[x].add_item(item);
            }
        }
    }

    monkeys.sort_by_cached_key(|m| m.plays);

    monkeys[monkeys.len() - 1].plays * monkeys[monkeys.len() - 2].plays
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, monkeys) = all_input(&buffer).unwrap();

    let p1 = solve_p1(monkeys.clone());
    let p2 = solve_p2(monkeys);

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
            parse_monkey(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            ),
            Ok((
                "",
                Monkey {
                    items: vec![79, 98],
                    operation: Operation::Times(19),
                    test: 23,
                    monkey_true: 2,
                    monkey_false: 3,
                    plays: 0,
                }
            ))
        );
    }

    #[test]
    fn test_sample() {
        let sample = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(solve(sample.as_bytes()), (10605, 2713310158));
    }
}
