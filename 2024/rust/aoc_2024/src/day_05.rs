use core::iter::Iterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub const TASK_SMALL: &str = "../../data/day_05_small.txt";
pub const TASK: &str = "../../data/day_05.txt";

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

fn is_valid(vec: &Vec<i32>, rules: &HashSet<(i32, i32)>) -> bool {
    vec.iter().enumerate().all(|(i, &value)| {
        vec[i + 1..]
            .iter()
            .all(|&other| !rules.contains(&(other, value)))
    })
}

fn split_content(content: &str) -> (&str, &str) {
    content.split_once("\n\n").expect("Invalid content format")
}

fn parse_rules(rules_unparsed: &str) -> HashSet<(i32, i32)> {
    rules_unparsed
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap()
        })
        .collect()
}

// Sort according to rules, i.e., for (x, y) in rules, x has to be before y.
fn sort_by_rules(mut vec: Vec<i32>, rules: &HashSet<(i32, i32)>) -> Vec<i32> {
    vec.sort_by(|&a, &b| {
        if rules.contains(&(a, b)) {
            std::cmp::Ordering::Less // `a` should come before `b`
        } else if rules.contains(&(b, a)) {
            std::cmp::Ordering::Greater // `b` should come before `a`
        } else {
            std::cmp::Ordering::Equal // No specific order for `a` and `b`
        }
    });
    vec
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = read_file_content(path);
    let (rules_unparsed, prints) = split_content(&content);
    let rules = parse_rules(&rules_unparsed);

    prints
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vec| is_valid(vec, &rules))
        .map(|vec| vec[vec.len() / 2])
        .sum()
}
pub fn solve_part_2(path: &str) -> i32 {
    let content = read_file_content(path);

    let (rules_unparsed, prints) = split_content(&content);
    let rules = parse_rules(&rules_unparsed);

    prints
        .lines()
        .map(|line| line.split(",").map(|c| c.parse::<i32>().unwrap()).collect())
        .filter(|vec| !is_valid(vec, &rules))
        .map(|vec| sort_by_rules(vec, &rules))
        .map(|vec| vec[vec.len() / 2])
        .sum()
}
