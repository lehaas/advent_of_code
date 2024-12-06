use core::iter::Iterator;
use std::fs;
use std::path::Path;

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

fn find_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {
        for (j, &value) in line.iter().enumerate() {
            if value == '^' {
                return (i, j);
            }
        }
    }
    panic!("Start position not found.")
}

fn get_next_direction(
    directions: &mut std::iter::Cycle<std::slice::Iter<(i32, i32)>>,
) -> (i32, i32) {
    *directions.next().expect("No direction")
}

fn is_within_bounds(pos: (usize, usize), direction: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let new_pos = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);
    new_pos.0 >= 0 && new_pos.0 < n && new_pos.1 >= 0 && new_pos.1 < m
}

fn get_new_position(pos: (usize, usize), direction: (i32, i32)) -> (usize, usize) {
    (
        (pos.0 as i32 + direction.0) as usize,
        (pos.1 as i32 + direction.1) as usize,
    )
}

fn count_pluses(map: Vec<Vec<char>>) -> i32 {
    map.iter()
        .map(|line| line.iter().filter(|&&c| c == '+').count() as i32)
        .sum()
}
fn count_steps(mut map: Vec<Vec<char>>, mut pos: (usize, usize)) -> i32 {
    let mut directions = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let mut direction: (i32, i32) = get_next_direction(&mut directions);

    loop {
        map[pos.0][pos.1] = '+';

        if !is_within_bounds(pos, direction, &map) {
            break;
        }

        let new_pos = get_new_position(pos, direction);

        match map[new_pos.0][new_pos.1] {
            '#' => direction = get_next_direction(&mut directions),
            '.' | '^' | '+' => pos = new_pos,
            _ => panic!("Unknown charcter"),
        }
    }
    // map.iter().for_each(|line| println!("{:?}", line));

    count_pluses(map)
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = read_file_content(path);

    let map = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let pos = find_start_position(&map);
    count_steps(map, pos)
}

pub fn solve_part_2(path: &str) -> i32 {
    let content = read_file_content(path);

    let map = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let pos = find_start_position(&map);
    count_steps(map, pos)
}
