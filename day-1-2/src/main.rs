use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut left_list = Vec::new();
    let mut right_list_occurrences = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut num_iter = line.split_whitespace();
        left_list.push(num_iter.next().ok_or("Number not found")?.parse::<u32>()?);
        let right_num = num_iter.next().ok_or("Number not found")?.parse::<u32>()?;
        right_list_occurrences
            .entry(right_num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let similarity: u32 = left_list
        .into_iter()
        .map(|n| n * right_list_occurrences.get(&n).unwrap_or(&0))
        .sum();

    println!("The similarity is {}", similarity);
    Ok(())
}
