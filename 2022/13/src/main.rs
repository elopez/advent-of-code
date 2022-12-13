extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::*,
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Item {
    Number(usize),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Number(x), Item::List(y)) => vec![Item::Number(*x)].cmp(y),
            (Item::List(x), Item::Number(y)) => x.cmp(&vec![Item::Number(*y)]),
            (Item::List(x), Item::List(y)) => x.cmp(y),
            (Item::Number(x), Item::Number(y)) => x.cmp(y),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn flatten(pairs: Vec<(Item, Item)>) -> Vec<Item> {
    pairs
        .into_iter()
        .flat_map(|tup| once(tup.0).chain(once(tup.1)))
        .collect()
}

fn parse_number(input: &str) -> IResult<&str, Item> {
    let (input, number) = map_res(digit1, usize::from_str)(input)?;
    Ok((input, Item::Number(number)))
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    alt((parse_number, parse_list))(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    let (input, list) =
        delimited(tag("["), separated_list0(tag(","), parse_item), tag("]"))(input)?;
    Ok((input, Item::List(list)))
}

fn parse_group(input: &str) -> IResult<&str, (Item, Item)> {
    let (input, list1) = parse_list(input)?;
    let (input, _) = multispace1(input)?;
    let (input, list2) = parse_list(input)?;

    Ok((input, (list1, list2)))
}

fn all_input(input: &str) -> IResult<&str, Vec<(Item, Item)>> {
    separated_list1(tag("\n\n"), parse_group)(input)
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let (_, pairs) = all_input(&buffer).unwrap();

    let mut part1 = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if a < b {
            part1 += i + 1;
        }
    }

    let mut list = flatten(pairs);

    let item1 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let item2 = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    list.extend(vec![item1.clone(), item2.clone()]);
    list.sort();

    let mut part2 = 1;
    for (i, item) in list.into_iter().enumerate() {
        if item == item1 {
            part2 *= i + 1
        }
        if item == item2 {
            part2 *= i + 1
        }
    }

    (part1, part2)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_list() {
        assert_eq!(
            parse_item("[3,4]"),
            Ok(("", Item::List(vec![Item::Number(3), Item::Number(4)])))
        );
    }

    #[test]
    fn test_parse_list_nest() {
        assert_eq!(
            parse_item("[3,[4,5]]"),
            Ok((
                "",
                Item::List(vec![
                    Item::Number(3),
                    Item::List(vec![Item::Number(4), Item::Number(5)])
                ])
            ))
        );
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_item("3"), Ok(("", Item::Number(3))));
    }

    #[test]
    fn test_vector_length_order() {
        let a = vec![1, 2];
        let b = vec![1, 2, 3];
        assert_eq!(a < b, true);
    }

    #[test]
    fn test_sample() {
        let sample = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(solve(sample.as_bytes()), (13, 140))
    }
}
