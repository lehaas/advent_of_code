use core::{iter::Iterator, panic};
use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::Vector2;
use ndarray_linalg::operator;
use regex::Regex;

use crate::utils;
use nom;

pub use nom::bytes::complete::tag;

pub const TASK_SMALL: &str = "../../data/day_17_small.txt";
pub const TASK: &str = "../../data/day_17.txt";

#[derive(Debug, Clone)]
struct Machine {
    A: u64,
    B: u64,
    C: u64,
}
impl Machine {
    fn _compute_combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!("Invalid operand {}", operand),
        }
    }
    fn compute(&mut self, program: &Vec<u64>) -> Vec<u64> {
        let mut instruction_pointer = 0;
        let mut output: Vec<u64> = Vec::new();

        while instruction_pointer < program.len() {
            let opcode = program[instruction_pointer];
            let operand = program[instruction_pointer + 1];
            // println!();
            // println!("{self:?}");
            // println!("IP {instruction_pointer:>5}, opcode: {opcode:>5}, operand: {operand:>5}, output: {output:?}");
            match opcode {
                // adv - A division with combo
                0 => self.A /= 2u64.pow(self._compute_combo_operand(operand).try_into().unwrap()),
                // bxl - B bitwise xor with literal
                1 => self.B ^= operand,
                // bst - B modulo with combo
                2 => self.B = self._compute_combo_operand(operand) % 8,
                // jnz - jump instruction pointer with literal
                3 => {
                    if self.A != 0 {
                        instruction_pointer = operand as usize;
                        continue; // skip increment by 2
                    }
                }
                // bxc - B xor C
                4 => self.B ^= self.C,
                // out - output with combo
                5 => {
                    output.push(self._compute_combo_operand(operand) % 8);
                    // if output != program[0..output.len()] {
                    //     // println!("returning {:?}", output);
                    //     return output;
                    // }
                }
                // bdv - B division with combo
                6 => {
                    self.B =
                        self.A / 2u64.pow(self._compute_combo_operand(operand).try_into().unwrap())
                }
                // cdv - C division with combo
                7 => {
                    self.C =
                        self.A / 2u64.pow(self._compute_combo_operand(operand).try_into().unwrap())
                }
                // not a valid instruction
                _ => panic!(),
            }
            instruction_pointer += 2;
        }
        output
    }
}

fn parse_machine(content: String) -> Option<(Machine, Vec<u64>)> {
    let mut lines = content.lines();
    let A = lines
        .next()?
        .split_once(": ")?
        .1
        .parse::<u64>()
        .expect("A failed");
    let B = lines
        .next()?
        .split_once(": ")?
        .1
        .parse::<u64>()
        .expect("B failed");
    let C = lines
        .next()?
        .split_once(": ")?
        .1
        .parse::<u64>()
        .expect("C failed");

    lines.next();
    let (_, program_str) = lines.next()?.split_once(": ")?;
    let program = program_str
        .split(",")
        .map(|x| x.parse::<u64>().expect("Cannot parse program"))
        .collect::<Vec<_>>();

    Some((Machine { A: A, B: B, C: C }, program))
}

pub fn solve_part_1(path: &str) -> Vec<u64> {
    let content = utils::read_file_content(path);
    let (mut machine, program) = parse_machine(content).expect("Failed to parse machine.");
    let output = machine.compute(&program);

    output
}

pub fn solve_part_2(path: &str) -> u64 {
    let content = utils::read_file_content(path);
    let (machine, program) = parse_machine(content).expect("Failed to parse machine.");

    let target = vec![2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 3, 5, 5, 3, 0];

    let mut current = HashSet::from([0]);
    for t in target.iter().rev() {
        println!("t = {}", t);
        println!("curr = {:?}", current);

        let mut next = HashSet::new();

        for a_running in current {
            for a in 0..8 {
                let mut current_machine: Machine = Machine {
                    A: a_running * 8 + a,
                    ..machine
                };

                let result = current_machine.compute(&program);
                if result[0] == *t {
                    next.insert(a_running * 8 + a);

                    println!("pushed {:?}", a_running * 8 + a);
                    println!("result = {:?}", result);
                }
            }
        }
        current = next;
    }

    *current.iter().next().unwrap() as u64
}
