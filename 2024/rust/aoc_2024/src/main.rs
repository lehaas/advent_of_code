mod day_07;
mod utils;
use day_07::{solve_part_1, solve_part_2, TASK, TASK_SMALL};

fn main() {
    println!("Result (small): {}", solve_part_1(TASK_SMALL));
    println!("Result: {}", solve_part_1(TASK));

    println!("Result (small): {}", solve_part_2(TASK_SMALL));
    println!("Result: {}", solve_part_2(TASK));
}
