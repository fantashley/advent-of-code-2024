use std::{
    cmp,
    io::{stdin, BufRead},
    num::ParseIntError,
};

fn main() {
    let stdin = stdin();
    let safe_reports_sum: u16 = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            let Ok(report) = line else {
                return None;
            };
            let Ok(levels): Result<Vec<u8>, ParseIntError> = report
                .split_whitespace()
                .map(|n| Ok(n.parse::<u8>()?))
                .collect()
            else {
                return None;
            };
            if is_safe(levels.into_iter()) {
                return Some(1);
            } else {
                return None;
            }
        })
        .sum();

    println!("The number of safe reports is {}", safe_reports_sum);
}

#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(levels: impl Iterator<Item = u8> + Clone) -> bool {
    let mut violations = count_violations(levels.clone());
    if violations <= 1 {
        return true;
    }
    // Try again without first element
    violations = count_violations(levels.clone().skip(1));
    if violations == 0 {
        return true;
    }
    // Try again without second element
    violations = count_violations(levels.clone().enumerate().filter_map(|(i, v)| {
        if i != 1 {
            Some(v)
        } else {
            None
        }
    }));
    violations == 0
}

fn count_violations(levels: impl Iterator<Item = u8>) -> u8 {
    let mut direction: Option<Direction> = None;
    let mut prev: Option<u8> = None;
    let mut violations = 0;

    for level in levels {
        if prev == None {
            prev = Some(level);
            continue;
        }
        let p = prev.unwrap();

        let Some(dir) = detected_direction(&p, &level) else {
            violations += 1;
            continue;
        };
        if direction == None {
            direction = Some(dir);
        } else if direction != Some(dir) {
            violations += 1;
            continue;
        }

        if p.abs_diff(level) > 3 {
            violations += 1;
            continue;
        }

        prev = Some(level);
    }

    if direction == None && violations < 2 {
        violations = 2
    }

    violations
}

fn detected_direction(a: &u8, b: &u8) -> Option<Direction> {
    match a.cmp(b) {
        cmp::Ordering::Less => Some(Direction::Increasing),
        cmp::Ordering::Equal => None,
        cmp::Ordering::Greater => Some(Direction::Decreasing),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_is_safe() {
        struct Test {
            report: Vec<u8>,
            is_safe: bool,
        }
        let tests = vec![
            Test {
                report: vec![9, 9, 8, 6, 2],
                is_safe: false,
            },
            Test {
                report: vec![34, 32, 36, 38, 41, 43, 46, 49],
                is_safe: true,
            },
            Test {
                report: vec![7, 6, 4, 2, 1],
                is_safe: true,
            },
            Test {
                report: vec![1, 2, 7, 8, 9],
                is_safe: false,
            },
            Test {
                report: vec![9, 7, 6, 2, 1],
                is_safe: false,
            },
            Test {
                report: vec![1, 3, 2, 4, 5],
                is_safe: true,
            },
            Test {
                report: vec![8, 6, 4, 4, 1],
                is_safe: true,
            },
            Test {
                report: vec![1, 3, 6, 7, 9],
                is_safe: true,
            },
        ];
        for test in tests {
            assert_eq!(
                test.is_safe,
                is_safe(test.report.clone().into_iter()),
                "incorrect safety for report {:?}",
                test.report
            )
        }
    }
}
