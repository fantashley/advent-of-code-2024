use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    process::exit,
};

fn main() {
    let mut word_search: Vec<Vec<char>> = Vec::new();
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {}", e);
            exit(1)
        });
        word_search.push(line.chars().collect());
    }

    let mut xmas_count = 0;
    for (yi, y) in word_search.iter().enumerate() {
        for (xi, x) in y.iter().enumerate() {
            if *x != 'A' {
                continue;
            }
            let mut has_one = false;
            for (dir, _) in DIRECTIONS {
                if find_xmas(xi, yi, dir, &word_search) {
                    if has_one {
                        xmas_count += 1;
                    }
                    has_one = true;
                }
            }
        }
    }

    println!("The total XMAS count is {}", xmas_count);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    NE,
    SE,
}

static DIRECTIONS: [(Direction, (i8, i8)); 2] = [(Direction::NE, (1, -1)), (Direction::SE, (1, 1))];

fn find_xmas(x: usize, y: usize, direction: Direction, grid: &Vec<Vec<char>>) -> bool {
    let dir_map = HashMap::from(DIRECTIONS);

    let (x1_add, y1_add) = dir_map[&direction];
    let char1 = match find_char(x, x1_add, y, y1_add, grid) {
        Some(c) => c,
        None => return false,
    };

    let (x2_add, y2_add) = (x1_add * -1, y1_add * -1);
    let char2 = match find_char(x, x2_add, y, y2_add, grid) {
        Some(c) => c,
        None => return false,
    };

    match (char1, char2) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false,
    }
}

fn find_char(x: usize, x_add: i8, y: usize, y_add: i8, grid: &Vec<Vec<char>>) -> Option<char> {
    let Some(x) = x.checked_add_signed(x_add.into()) else {
        return None;
    };
    let Some(y) = y.checked_add_signed(y_add.into()) else {
        return None;
    };

    match grid.get(y) {
        Some(row) => match row.get(x) {
            Some(c) => Some(*c),
            None => None,
        },
        None => None,
    }
}
