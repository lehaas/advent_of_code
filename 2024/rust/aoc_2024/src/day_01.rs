use crate::utils;
use std::fs;
use std::path::Path;

pub const TASK_SMALL: &str = "../../data/day_01_small.txt";
pub const TASK: &str = "../../data/day_01.txt";

/// Parse a string of two integers, e.g., "15   16\n48   16\n" into two lists of integers.
fn parse_content(content: &str) -> (Vec<i32>, Vec<i32>) {
    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split("   ").map(|x| x.parse::<i32>());
            match (parts.next(), parts.next()) {
                (Some(Ok(left)), Some(Ok(right))) => Some((left, right)),
                _ => None, // Skip lines that don't parse correctly
            }
        })
        .unzip()
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = utils::read_file_content(path);

    let (mut left, mut right) = parse_content(&content);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}
pub fn solve_part_2(path: &str) -> usize {
    let content = utils::read_file_content(path);
    let (left, right) = parse_content(&content);

    let res = left
        .iter()
        .map(|v| {
            let count = right.iter().filter(|&n| *n == *v).count();
            (*v as usize) * count
        })
        .sum();

    res
}
