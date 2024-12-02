use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut num_iter = line.split_whitespace();
        left_list.push(num_iter.next().ok_or("Number not found")?.parse::<u32>()?);
        right_list.push(num_iter.next().ok_or("Number not found")?.parse::<u32>()?);
    }

    left_list.sort_unstable();
    right_list.sort_unstable();

    let diff_sum: u32 = left_list
        .into_iter()
        .zip(right_list.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("The total is {}", diff_sum);
    Ok(())
}
