use core::iter::Iterator;
use std::fmt::Debug;
use std::fs;
use std::ops;
use std::path::Path;

pub const task_small_path: &str = "../../data/day_07_small.txt";
pub const task_path: &str = "../../data/day_07.txt";

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

fn parse_line(line: &str) -> Option<(u64, Vec<u64>)> {
    let (x, xs) = line.split_once(": ")?;
    Some((
        x.parse::<_>().expect("String is not a valid integer."),
        xs.split(" ")
            .map(|y| y.parse::<_>().expect("String is not a valid integer."))
            .collect::<Vec<_>>(),
    ))
}

fn is_computable(target: u64, candidates: &Vec<u64>) -> Option<u64> {
    // _is_computable(target, &candidates, ops::Add::add, 0)

    let num_bits = candidates.len() - 1;
    // println!("target = {}, candidates = {:?}", target, candidates);
    // println!("{}", 1 << num_bits);

    let bit_representations = (0..(2_usize.pow(num_bits as u32)))
        .map(|num| format!("{:0width$b}", num, width = num_bits));

    for bit_representation in bit_representations {
        let (x, xs) = candidates.split_at(1);
        let start = x[0];
        let res = xs
            .iter()
            .zip(
                bit_representation
                    .chars()
                    .map(|char| char.to_digit(10))
                    .collect::<Vec<_>>()
                    .iter(),
            )
            .fold(start, |acc, (x, op)| match op {
                Some(0) => acc + x,
                Some(1) => acc * x,
                _ => panic!("Invalid operator."),
            });
        // println!("bit {}, res {}", bit_representation, res);

        if res == target {
            return Some(target);
        }
    }
    None
}

pub fn solve_part_1(path: &str) -> u64 {
    let content = read_file_content(path);

    content
        .lines()
        .filter_map(|line| parse_line(line))
        .filter_map(|(target, values)| is_computable(target, &values))
        .sum()
}

fn to_ternary(mut num: usize, width: usize) -> String {
    let mut ternary = String::new();

    // Convert the number to base 3
    while num > 0 {
        ternary.push_str(&(num % 3).to_string());
        num /= 3;
    }

    // Reverse the string to get the correct order
    ternary = ternary.chars().rev().collect();

    // Pad with leading zeroes to the desired width
    format!("{:0>width$}", ternary, width = width)
}

fn is_computable2(target: u64, candidates: &Vec<u64>) -> Option<u64> {
    // _is_computable(target, &candidates, ops::Add::add, 0)

    let num_bits = candidates.len() - 1;
    // println!("target = {}, candidates = {:?}", target, candidates);
    // println!("{}", 1 << num_bits);

    let bit_representations =
        (0..(3_usize.pow(num_bits as u32))).map(|num| to_ternary(num, num_bits));

    for bit_representation in bit_representations {
        let (x, xs) = candidates.split_at(1);
        let start = x[0];
        let res = xs
            .iter()
            .zip(
                bit_representation
                    .chars()
                    .map(|char| char.to_digit(10))
                    .collect::<Vec<_>>()
                    .iter(),
            )
            .fold(start, |acc, (x, op)| match op {
                Some(0) => acc + x,
                Some(1) => acc * x,
                Some(2) => acc * 10_u64.pow(x.to_string().len() as u32) + x,
                _ => panic!("Invalid operator."),
            });
        // println!("bit {}, res {}", bit_representation, res);

        if res == target {
            return Some(target);
        }
    }
    None
}

pub fn solve_part_2(path: &str) -> u64 {
    let content = read_file_content(path);

    content
        .lines()
        .filter_map(|line| parse_line(line))
        .filter_map(|(target, values)| is_computable2(target, &values))
        .sum()
}
