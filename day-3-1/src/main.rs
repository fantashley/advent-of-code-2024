use std::{
    io::{stdin, Read},
    process::exit,
};

fn main() {
    let mut buffer = String::new();
    let stdin = stdin();
    stdin
        .lock()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {}", e);
            exit(1);
        });

    let mut total: u32 = 0;
    for (i, _) in buffer.match_indices("mul(") {
        let product: u32 = match parse_numbers(buffer.chars().skip(i + 4)) {
            Some((n1, n2)) => n1 as u32 * n2 as u32,
            None => continue,
        };
        total += product as u32;
    }

    println!("The sum of products is {}", total);
}

fn parse_numbers(mut input: impl Iterator<Item = char>) -> Option<(u16, u16)> {
    let mut nums: Vec<char> = vec![];

    // First number
    loop {
        let next = match input.next() {
            Some(c) => c,
            None => break,
        };
        if next.is_digit(10) {
            nums.push(next);
        } else if next == ',' {
            break;
        } else {
            return None;
        }
    }

    let Some(first_num) = check_and_parse_num(nums) else {
        return None;
    };

    let mut nums: Vec<char> = vec![];

    // Second number
    loop {
        let next = match input.next() {
            Some(c) => c,
            None => break,
        };
        if next.is_digit(10) {
            nums.push(next);
        } else if next == ')' {
            break;
        } else {
            return None;
        }
    }

    let Some(second_num) = check_and_parse_num(nums) else {
        return None;
    };

    Some((first_num, second_num))
}

fn check_and_parse_num(nums: Vec<char>) -> Option<u16> {
    if nums.is_empty() || nums.len() > 3 {
        return None;
    }
    let parsed = String::from_iter(nums);
    parsed.parse::<u16>().ok()
}
