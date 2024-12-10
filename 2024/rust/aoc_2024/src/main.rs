mod day_07;
use day_07::{solve_part_1, solve_part_2, task_path, task_small_path};

fn main() {
    println!("Result (small): {}", solve_part_1(task_small_path));
    println!("Result: {}", solve_part_1(task_path));

    println!("Result (small): {}", solve_part_2(task_small_path));
    println!("Result: {}", solve_part_2(task_path));
}
