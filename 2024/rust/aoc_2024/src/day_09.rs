use core::iter::Iterator;
use std::fmt::Debug;
use std::fs;
use std::ops;
use std::path::Path;

use itertools::max;

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

pub fn solve_part_1(path: &str) -> i64 {
    let content = read_file_content(path);

    let mut blocks = parse_blocks(content);

    let mut i = 0;
    let mut j = blocks.len() - 1;
    while i < j {
        if blocks[i] == -1 {
            blocks.swap(i, j);
            while blocks[j] == -1 {
                j -= 1;
            }
        }
        i += 1
    }
    compute_checksum(blocks)
}

fn compute_checksum(blocks: Vec<i32>) -> i64 {
    blocks
        .into_iter()
        .enumerate()
        .map(|(i, value)| match value {
            -1 => 0,
            _ => i as i64 * value as i64,
        })
        .sum::<i64>()
}

fn parse_blocks(content: String) -> Vec<i32> {
    let mut blocks = content
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, count)| {
            // println!("{} {}", i, count);
            match i % 2 {
                0 => vec![(i / 2) as i32; count as usize],
                1 => vec![-1; count as usize],
                _ => panic!("modulo 2 should not result in values outside [0,1]."),
            }
        })
        .collect::<Vec<_>>();
    blocks
}

pub fn solve_part_2(path: &str) -> i64 {
    let content = read_file_content(path);

    let mut blocks = parse_blocks(content);

    println!("{:?}", blocks);

    let max_id = blocks.iter().max().copied().unwrap_or(0);
    for id in (1..=max_id).rev() {
        if let Some(i_id) = blocks.iter().position(|&block| block == id) {
            let occurrences = compute_occurences(&blocks, i_id, id);
            if let Some(j) = find_next_free_block(&blocks, occurrences) {
                if j >= i_id {
                    continue;
                }
                for k in 0..occurrences {
                    blocks.swap(i_id + k, j + k);
                }
            }
        }
    }
    println!("{:?}", blocks);
    compute_checksum(blocks)
}

fn compute_occurences(blocks: &Vec<i32>, mut start: usize, value: i32) -> usize {
    let mut count = 0;
    while start < blocks.len() && blocks[start] == value {
        count += 1;
        start += 1;
    }
    count
}

fn find_next_free_block(blocks: &Vec<i32>, count: usize) -> Option<usize> {
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            continue;
        }

        let mut c = 0;
        while i + c < blocks.len() && blocks[i + c] == -1 {
            c += 1;
        }
        if c >= count {
            return Some(i);
        }
    }
    None
}
