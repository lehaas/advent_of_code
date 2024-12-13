use core::iter::Iterator;

use itertools::Itertools;
use nalgebra::Vector2;
use regex::Regex;

use crate::utils;

pub const TASK_SMALL: &str = "../../data/day_13_small.txt";
pub const TASK: &str = "../../data/day_13.txt";

fn parse_puzzle(task: &str) -> (Vector2<i64>, Vector2<i64>, Vector2<i64>) {
    let lines = task.lines().collect_vec();

    (
        parse_button(lines[0]),
        parse_button(lines[1]),
        parse_prize(lines[2]),
    )
}

fn parse_button(line: &str) -> nalgebra::Vector2<i64> {
    let re = Regex::new(r"Button [A-B]: X\+([0-9]+), Y\+([0-9]+)").unwrap();

    let caps = re.captures(line).unwrap();
    let A = Vector2::new(
        caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    );
    A
}

fn parse_prize(line: &str) -> nalgebra::Vector2<i64> {
    let re = Regex::new(r"Prize: X=([0-9]*), Y=([0-9]*)").unwrap();

    let caps = re.captures(line).unwrap();
    let A = Vector2::new(
        caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
    );
    A
}

fn solve_puzzle(task: &str) -> Option<i64> {
    let (A, B, prize) = parse_puzzle(task);

    for i in 0..=100 {
        let target = prize - i * A;
        let j = target.x / B.x;
        if target == B * j {
            return Some(i * 3 + j);
        }
    }
    None
}

pub fn solve_part_1(path: &str) -> i64 {
    let content = utils::read_file_content(path);

    content
        .split("\n\n")
        .filter_map(|line| solve_puzzle(line))
        .sum()
}

fn cast_matrix_i64_to_f64(mat: nalgebra::Matrix2<i64>) -> nalgebra::Matrix2<f64> {
    let (rows, cols) = mat.shape();
    let mut result = nalgebra::Matrix2::zeros();

    for i in 0..rows {
        for j in 0..cols {
            result[(i, j)] = mat[(i, j)] as f64;
        }
    }

    result
}

fn solve_puzzle2(task: &str) -> Option<i64> {
    let (A, B, prize) = parse_puzzle(task);
    let prize = prize.add_scalar(10000000000000);

    // Use Cramer's rule to solve the linear equation system
    let D = cast_matrix_i64_to_f64(nalgebra::Matrix2::from_columns(&[A, B])).determinant();
    let Dx = cast_matrix_i64_to_f64(nalgebra::Matrix2::from_columns(&[prize, B])).determinant();
    let Dy = cast_matrix_i64_to_f64(nalgebra::Matrix2::from_columns(&[A, prize])).determinant();

    let x = Vector2::new(Dx / D, Dy / D);

    println!("-----");
    println!("{}, {}", x.x, x.y);
    println!("{}, {}", x.x.trunc(), x.y.trunc());
    println!("{}, {}", x.x == x.x.trunc(), x.y == x.y.trunc());
    println!("-----");

    // we are only interested in integer solutions
    if x.x == x.x.trunc() && x.y == x.y.trunc() {
        return Some((x.x * 3.0 + x.y) as i64);
    }
    None
}

pub fn solve_part_2(path: &str) -> i64 {
    let content = utils::read_file_content(path);

    content
        .split("\n\n")
        .filter_map(|line| solve_puzzle2(line))
        .inspect(|x| println!("made it through filter: {x}"))
        .sum()
}
