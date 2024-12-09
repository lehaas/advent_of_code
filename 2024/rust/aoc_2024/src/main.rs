mod day_09;
use day_09::{solve_part_1, solve_part_2};

fn main() {
    let task_small_path = "../../data/day_09_small.txt";
    let task_path: &str = "../../data/day_09.txt";

    println!("Result (small): {}", solve_part_1(task_small_path));
    println!("Result: {}", solve_part_1(task_path));

    println!("Result (small): {}", solve_part_2(task_small_path));
    println!("Result: {}", solve_part_2(task_path));
}
