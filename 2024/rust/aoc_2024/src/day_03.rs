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

pub fn solve_part_1(path: &str) -> i32 {
    let content = read_file_content(path);

    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();

    let res = content
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| match (c["x"].parse::<i32>(), c["y"].parse::<i32>()) {
                    (Result::Ok(x), Result::Ok(y)) => x * y,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum();
    res
}

pub fn solve_part_2(path: &str) -> usize {
    0
}
