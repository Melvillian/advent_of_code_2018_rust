use std::fs;
use std::collections::HashSet;
use crate::util::{get_input_as_vec, get_input_file_as_string};

pub fn aoc_1_1_improved() {
    let frequencies = get_input_file_as_string("aoc_1_input.txt");

    let sum : i32 = get_input_as_vec(frequencies)
        .iter()
        .sum();
    println!("aoc_1_1: {}", sum);
}

/// looks for duplicate frequencies in the input list
pub fn aoc_1_2_improved() {
    let mut set = HashSet::new();
    let frequencies = get_input_file_as_string("aoc_1_input.txt");
    let frequencies = get_input_as_vec(frequencies);

    let mut sum : i32 = 0;
    set.insert(sum);

    for freq in frequencies.iter().cycle() {
        sum+= freq;

        if !set.insert(sum) {
            break;
        }
    }

    println!("aoc_1_2: {}", sum);
}
