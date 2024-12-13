use core::iter::Iterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::utils;

pub const TASK_SMALL: &str = "../../data/day_10_small.txt";
pub const TASK: &str = "../../data/day_10.txt";

fn print_map<T: std::fmt::Debug>(map: &Vec<Vec<T>>) {
    map.iter().for_each(|line: &Vec<T>| println!("{:?}", line));
}
pub fn solve_part_1(path: &str) -> usize {
    let content = utils::read_file_content(path);

    let map: Vec<Vec<u32>> = parse_map(content);

    let mut targets: Vec<Vec<HashSet<(usize, usize)>>> =
        vec![vec![HashSet::new(); map[0].len()]; map.len()];

    for i in (0..map.len()) {
        for j in (0..map[0].len()) {
            if map[i][j] == 9 {
                targets[i][j].insert((i, j));
            }
        }
    }

    print_map(&targets);
    for k in (0..9).rev() {
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] != k {
                    continue;
                }
                if i > 0 && (map[i - 1][j] == k + 1) {
                    let t = targets[i - 1][j].clone();
                    targets[i][j].extend(t);
                }
                if i < map.len() - 1 && (map[i + 1][j] == k + 1) {
                    let t = targets[i + 1][j].clone();
                    targets[i][j].extend(t);
                }
                if j > 0 && (map[i][j - 1] == k + 1) {
                    let t = targets[i][j - 1].clone();
                    targets[i][j].extend(t);
                }
                if j < map[0].len() - 1 && (map[i][j + 1] == k + 1) {
                    let t = targets[i][j + 1].clone();
                    targets[i][j].extend(t);
                }
            }
        }
    }

    print_map(&targets);

    (0..map.len())
        .map(|i| {
            (0..map[0].len())
                .filter(|&j| map[i][j] == 0)
                .map(|j| targets[i][j].len())
                .sum::<usize>()
        })
        .sum()
}

pub fn solve_part_2(path: &str) -> usize {
    let content = utils::read_file_content(path);

    let map = parse_map(content);
    print_map(&map);

    let mut targets: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 9 {
                targets[i][j] = 1;
            }
        }
    }

    print_map(&targets);
    for k in (0..9).rev() {
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] != k {
                    continue;
                }
                if i > 0 && (map[i - 1][j] == k + 1) {
                    targets[i][j] += targets[i - 1][j];
                }
                if i < map.len() - 1 && (map[i + 1][j] == k + 1) {
                    targets[i][j] += targets[i + 1][j];
                }
                if j > 0 && (map[i][j - 1] == k + 1) {
                    targets[i][j] += targets[i][j - 1];
                }
                if j < map[0].len() - 1 && (map[i][j + 1] == k + 1) {
                    targets[i][j] += targets[i][j + 1];
                }
            }
        }
    }

    print_map(&targets);

    (0..map.len())
        .map(|i| {
            (0..map[0].len())
                .filter(|&j| map[i][j] == 0)
                .map(|j| targets[i][j])
                .sum::<usize>()
        })
        .sum()
}

fn parse_map(content: String) -> Vec<Vec<u32>> {
    let mut map = content
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    map
}
