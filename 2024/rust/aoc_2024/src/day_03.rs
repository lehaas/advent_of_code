use crate::utils;
use core::iter::Iterator;
use regex::Regex;
use std::fs;
use std::path::Path;

pub const TASK_SMALL: &str = "../../data/day_03_small.txt";
pub const TASK: &str = "../../data/day_03.txt";

/// Parse the string ignoring the memory corruption:
/// - Ignore everything except for mul(x,y) for x and y, 3 character numbers
/// - return the sum of the product of (x * y) s
fn parse_program(line: &str) -> i32 {
    let re = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();

    re.captures_iter(line)
        .map(|c| match (c["x"].parse::<i32>(), c["y"].parse::<i32>()) {
            (Result::Ok(x), Result::Ok(y)) => x * y,
            _ => 0,
        })
        .sum::<i32>()
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = utils::read_file_content(path);
    parse_program(&content)
}

pub fn solve_part_2(path: &str) -> i32 {
    let content = utils::read_file_content(path);

    // filter out parts of the program following a don't until the next do
    let filtered_program: String = content
        .split("don't")
        .enumerate()
        .map(|(i, chunk)| match (i, chunk) {
            // the first entry has an implicit "do"
            (0, chunk) => chunk.to_string(),
            // keep the entries after "do"
            (_, chunk) => chunk.split("do").skip(1).collect::<String>(),
        })
        .collect::<String>();

    parse_program(&filtered_program)
}
