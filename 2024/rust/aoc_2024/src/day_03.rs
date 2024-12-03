use core::iter::Iterator;
use regex::Regex;
use std::fs;
use std::path::Path;

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

fn parse_mul(chars: &[char]) -> Option<i32> {
    let s: String = chars.iter().collect();

    let re = Regex::new(r"^mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let cap = re.captures(&s);
    // println!("{:?}", cap);
    match cap {
        Some(cap) => match (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
            (Result::Ok(x), Result::Ok(y)) => {
                // println!("{},{}", x, y);
                Some(x * y)
            }
            _ => None,
        },
        None => None,
    }
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = read_file_content(path);
    let res: i32 = content
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .windows(5 + 2 * 3)
                .filter_map(|s| parse_mul(s))
                .sum::<i32>()
        })
        .sum();
    res
}

pub fn solve_part_2(path: &str) -> usize {
    0
}
