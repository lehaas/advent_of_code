mod day_01;

use day_01::{solve_part_1, solve_part_2};

fn main() {
    println!(
        "Result (small): {}",
        solve_part_1("../../data/day_01_small.txt")
    );
    println!("Result: {}", solve_part_1("../../data/day_01.txt"));

    println!(
        "Result (small): {}",
        solve_part_2("../../data/day_01_small.txt")
    );
    println!("Result: {}", solve_part_2("../../data/day_01.txt"));
}
