use std::io::{self, BufRead};
use priority_queue::PriorityQueue;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lock().lines();
    let mut sum = 0;
    let mut pq = PriorityQueue::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            pq.push(sum, sum);
            sum = 0;
            continue;
        }

        let n = last_input.parse::<u128>().unwrap();
        sum += n;
    }

    pq.push(sum, sum);

    let mut total = 0;
    for (item, _) in pq.into_sorted_iter().take(3) {
        println!("Most calories: {}", item);
        total += item;
    }

    println!("Sum: {}", total);

    Ok(())
}