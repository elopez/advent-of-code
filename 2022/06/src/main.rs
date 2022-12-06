use std::collections::HashSet;
use std::io::{self, BufRead};

fn look_contiguous(buffer: &str, len: usize) -> usize {
    let mut start = len - 1;
    for i in 0..buffer.len() - len {
        let hs: HashSet<&u8> = HashSet::from_iter(buffer[i..i + len].as_bytes());
        if hs.len() == len {
            start = i + len;
            break;
        }
    }
    start
}

fn solve(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let sop = look_contiguous(&buffer, 4);
    let som = look_contiguous(&buffer, 14);

    (sop, som)
}

fn main() {
    let (p1, p2) = solve(io::stdin().lock());
    println!("Total: {p1} / {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes()), (7, 19));
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes()), (5, 23));
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg".as_bytes()), (6, 23));
        assert_eq!(
            solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes()),
            (10, 29)
        );
        assert_eq!(
            solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes()),
            (11, 26)
        );
    }
}
