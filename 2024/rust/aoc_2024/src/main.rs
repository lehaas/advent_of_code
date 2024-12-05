mod day_05;
use day_05::{solve_part_1, solve_part_2};

fn main() {
    let task_small_path = "../../data/day_05_small.txt";
    let task_path: &str = "../../data/day_05.txt";

    println!("Result (small): {}", solve_part_1(task_small_path));
    println!("Result: {}", solve_part_1(task_path));

    println!("Result (small): {}", solve_part_2(task_small_path));
    println!("Result: {}", solve_part_2(task_path));
}
