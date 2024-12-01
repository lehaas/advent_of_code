use std::fs;
use std::path::Path;

fn read_file_contents(file_path: impl AsRef<Path>) -> String {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
}

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
    let content = read_file_contents(path);

    let (mut left, mut right) = parse_content(&content);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}
pub fn solve_part_2(path: &str) -> usize {
    let content = read_file_contents(path);
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
