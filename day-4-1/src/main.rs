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
            if *x != 'X' {
                continue;
            }
            for (dir, _) in DIRECTIONS {
                if find_xmas(xi, yi, dir, &word_search) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("The total XMAS count is {}", xmas_count);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

static DIRECTIONS: [(Direction, (i8, i8)); 8] = [
    (Direction::N, (0, -1)),
    (Direction::NE, (1, -1)),
    (Direction::E, (1, 0)),
    (Direction::SE, (1, 1)),
    (Direction::S, (0, 1)),
    (Direction::SW, (-1, 1)),
    (Direction::W, (-1, 0)),
    (Direction::NW, (-1, -1)),
];

fn find_xmas(x: usize, y: usize, direction: Direction, grid: &Vec<Vec<char>>) -> bool {
    let dir_map = HashMap::from(DIRECTIONS);
    let (x_add, y_add) = dir_map[&direction];
    let mut curr_x = x;
    let mut curr_y = y;

    for c in "MAS".chars() {
        curr_x = match curr_x.checked_add_signed(x_add.into()) {
            Some(x) => x,
            None => return false,
        };
        curr_y = match curr_y.checked_add_signed(y_add.into()) {
            Some(y) => y,
            None => return false,
        };
        let Some(row) = grid.get(curr_y) else {
            return false;
        };
        let Some(val) = row.get(curr_x) else {
            return false;
        };
        if *val != c {
            return false;
        }
    }

    true
}
