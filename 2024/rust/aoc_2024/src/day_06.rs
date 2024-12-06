use core::iter::Iterator;
use std::fs;
use std::path::Path;

/// Read the content of a file into a String.
fn read_file_content(file_path: impl AsRef<Path>) -> String {
    let content: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    content
}

fn find_start_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, line) in map.iter().enumerate() {
        for (j, &value) in line.iter().enumerate() {
            if value == '^' {
                return Some((i, j));
            }
        }
    }
    None
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

fn count_minuses(map: &Vec<Vec<char>>) -> i32 {
    map.iter()
        .map(|line| line.iter().filter(|&&c| c == '-' || c == '+').count() as i32)
        .sum()
}

#[derive(Debug, PartialEq)]
enum State {
    LoopFound,
    ExitedMap,
}

fn found_loop(pos: (usize, usize), direction: (i32, i32), map: &Vec<Vec<char>>) -> bool {
    let new_pos: (usize, usize) = get_new_position(pos, direction);
    map[pos.0][pos.1] == '+' && map[new_pos.0][new_pos.1] == '#'
}

fn predict_guard_path(map: &mut Vec<Vec<char>>, mut pos: (usize, usize)) -> State {
    let mut directions = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let mut direction: (i32, i32) = get_next_direction(&mut directions);

    loop {
        if found_loop(pos, direction, &map) {
            return State::LoopFound;
        }
        map[pos.0][pos.1] = '-';

        if !is_within_bounds(pos, direction, &map) {
            break;
        }

        let mut new_pos: (usize, usize) = get_new_position(pos, direction);

        while map[new_pos.0][new_pos.1] == '#' {
            map[pos.0][pos.1] = '+';
            direction = get_next_direction(&mut directions);
            new_pos = get_new_position(pos, direction);
        }
        pos = new_pos;
    }

    State::ExitedMap
}

fn print_map(map: &Vec<Vec<char>>) {
    map.iter()
        .for_each(|line: &Vec<char>| println!("{:?}", line));
}

pub fn solve_part_1(path: &str) -> i32 {
    let content = read_file_content(path);

    let mut map = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let start_position = find_start_position(&map).expect("Guard position not found");
    predict_guard_path(&mut map, start_position);
    count_minuses(&map)
}

pub fn solve_part_2(path: &str) -> i32 {
    let content = read_file_content(path);

    let original_map = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let start_position = find_start_position(&original_map).expect("Guard position not found");

    let blockage_positions = determine_blockag_start_points(&original_map, start_position);

    blockage_positions
        .iter()
        .filter(|(i, j)| blockage_creates_loop(&original_map, i, j, start_position))
        .count() as i32
}

fn blockage_creates_loop(
    original_map: &Vec<Vec<char>>,
    i: &usize,
    j: &usize,
    start_position: (usize, usize),
) -> bool {
    let mut map: Vec<Vec<char>> = original_map.clone();
    map[*i][*j] = '#';
    predict_guard_path(&mut map, start_position) == State::LoopFound
}

fn determine_blockag_start_points(
    original_map: &Vec<Vec<char>>,
    start_position: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut tracked_map = original_map.clone();
    assert!(predict_guard_path(&mut tracked_map, start_position) == State::ExitedMap);
    let blockage_positions: Vec<_> = tracked_map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter().enumerate().filter_map(move |(j, &value)| {
                if value == '+' || value == '-' {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .collect();
    blockage_positions
}
