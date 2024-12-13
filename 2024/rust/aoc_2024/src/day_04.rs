use crate::utils;
use core::iter::Iterator;
use regex::Regex;
use std::fs;
use std::path::Path;

pub const TASK_SMALL: &str = "../../data/day_04_small.txt";
pub const TASK: &str = "../../data/day_04.txt";

pub fn solve_part_1(path: &str) -> i32 {
    let content = utils::read_file_content(path);

    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let directions: Vec<(i32, i32)> = vec![
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut count = 0;
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            for (x_shift, y_shift) in directions.iter() {
                match search_direction(x as i32, x_shift, y as i32, y_shift, &matrix) {
                    Some(_) => count += 1,
                    None => (),
                }
            }
        }
    }

    count
}

fn search_direction(
    mut x: i32,
    x_shift: &i32,
    mut y: i32,
    y_shift: &i32,
    matrix: &Vec<Vec<char>>,
) -> Option<()> {
    let word = ['X', 'M', 'A', 'S'];
    let shape = (matrix.len() as i32, matrix[0].len() as i32);

    for c in word {
        if x < 0 || x >= shape.0 || y < 0 || y >= shape.1 {
            return None;
        }

        if matrix[x as usize][y as usize] != c {
            return None;
        }

        x += x_shift;
        y += y_shift;
    }
    Some(())
}

fn is_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Option<i32> {
    if matrix[x][y] != 'A' {
        return None;
    }

    let c = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        .iter()
        .filter_map(|&(x_shift, y_shift)| {
            let x1 = x.checked_add_signed(x_shift)?;
            let y1 = y.checked_add_signed(y_shift)?;
            let x2 = x.checked_add_signed(-x_shift)?;
            let y2 = y.checked_add_signed(-y_shift)?;

            if matrix[x1][y1] == 'M' && matrix[x2][y2] == 'S' {
                Some(())
            } else {
                None
            }
        })
        .count();
    match c {
        2 => Some(1),
        _ => None,
    }
}

pub fn solve_part_2(path: &str) -> i32 {
    let content = utils::read_file_content(path);

    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let n = matrix.len();
    let m = matrix[0].len();
    (1..n - 1)
        .flat_map(|x| (1..m - 1).map(move |y| (x, y)))
        .filter_map(|(x, y)| is_xmas(&matrix, x, y))
        .count() as i32
}
