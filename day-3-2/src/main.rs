use std::{
    collections::HashMap,
    io::{stdin, Read},
    iter::Peekable,
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
    let mut chars = buffer.chars().peekable();
    let mut multiply_on = true;

    loop {
        if chars.peek() == None {
            break;
        }
        let next = &chars.next_if(|n| *n != 'm' && *n != 'd');
        if *next != None {
            continue;
        }
        match parse_instruction(&mut chars) {
            None => continue,
            Some(Instruction::Do) => multiply_on = true,
            Some(Instruction::Dont) => multiply_on = false,
            Some(Instruction::Multiply) => {
                let product: u32 = match parse_numbers(&mut chars) {
                    Some((n1, n2)) => n1 as u32 * n2 as u32,
                    None => continue,
                };
                if multiply_on {
                    total += product;
                }
            }
        }
    }

    println!("The sum of products is {}", total);
}

#[derive(Clone)]
enum Instruction {
    Multiply,
    Do,
    Dont,
}

fn parse_instruction<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> Option<Instruction> {
    let instructions = HashMap::from([
        ("do()".to_string(), Instruction::Do),
        ("don't()".to_string(), Instruction::Dont),
        ("mul(".to_string(), Instruction::Multiply),
    ]);

    let mut seen = vec![input.next().unwrap_or_default()];
    loop {
        let next = match input.peek() {
            None => return None,
            Some(c) => c,
        };
        seen.push(*next);
        let curr_word: String = seen.iter().collect();
        let mut has_match = false;
        for (i, e) in &instructions {
            if curr_word.eq(i) {
                input.next();
                return Some(e.clone());
            }
            if i.starts_with(&curr_word) {
                has_match = true;
                input.next();
                break;
            }
        }
        if !has_match {
            return None;
        }
    }
}

fn parse_numbers<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> Option<(u16, u16)> {
    let mut nums: Vec<char> = vec![];

    // First number
    loop {
        let next = match input.next_if(|n| n.is_digit(10) || *n == ',') {
            Some(c) => c,
            None => return None,
        };
        if next.is_digit(10) {
            nums.push(next.clone());
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
        let next = match input.next_if(|n| n.is_digit(10) || *n == ')') {
            Some(c) => c,
            None => return None,
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
