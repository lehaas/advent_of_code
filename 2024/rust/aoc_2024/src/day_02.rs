use crate::utils;
use std::fs;
use std::path::Path;

pub const TASK_SMALL: &str = "../../data/day_02_small.txt";
pub const TASK: &str = "../../data/day_02.txt";

fn parse_content(contents: String) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("TODO"))
                .collect();
            parts
        })
        .collect()
}

pub fn solve_part_1(path: &str) -> usize {
    let contents = utils::read_file_content(path);
    let parsed_content = parse_content(contents);

    parsed_content.iter().filter(|line| is_safe(line)).count()
}

/// Check if a list is safe, i.e., the list is sorted and all neighbouring entries have a distance between 1 and 3
fn is_safe(line: &Vec<i32>) -> bool {
    // check that each line is sorted
    match line.is_sorted_by(|a, b| a > b) || line.is_sorted_by(|a, b| a < b) {
        // check that the neighbouring distance is between 1 and 3
        true => line.windows(2).all(|w| {
            let v = (w[0] - w[1]).abs();
            1 <= v && v <= 3
        }),
        false => false,
    }
}

/// Generate all possible lists by dropping one element of the list
fn generate_removed_lists(input: &Vec<i32>) -> Vec<Vec<i32>> {
    (0..input.len())
        .map(|i| {
            let mut new_vec = input.clone();
            new_vec.remove(i);
            new_vec
        })
        .collect()
}

pub fn solve_part_2(path: &str) -> usize {
    let contents = utils::read_file_content(path);
    let parsed_content = parse_content(contents);
    parsed_content
        .iter()
        .filter(|line| {
            is_safe(line)
                || generate_removed_lists(line)
                    .iter()
                    .any(|replacement| is_safe(replacement))
        })
        .count()
}
