use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[allow(dead_code)]
/// Sums the numeric values on each line in the input
pub fn aoc_1_1() {
    let frequencies = get_input_file_as_string("aoc_1_input.txt");

    let mut sum : i32 = 0;
    for line in frequencies.lines() {
        let (sign, num) = line.split_at(1);
        let num : i32 = num.parse().unwrap();
        match sign {
            "+" => sum+= num,
            "-" => sum-= num,
            _ => panic!("unknown symbol {}", sign),
        }
    }

    println!("aoc_1_1: {}", sum);
}

#[allow(dead_code)]
/// looks for duplicate frequencies in the input list
pub fn aoc_1_2() {
    let mut map = HashSet::new();
    let frequencies = get_input_file_as_string("aoc_1_input.txt");

    let mut sum : i32 = 0;

    'outer: loop {
        'inner: for line in frequencies.lines() {
            let is_duplicate = map.insert(sum);

            if !is_duplicate {
                break 'outer;
            }

            let (sign, num) = line.split_at(1);
            let num : i32 = num.parse().unwrap();
            match sign {
                "+" => sum+= num,
                "-" => sum-= num,
                _ => panic!("unknown symbol {}", sign),
            }
        }
    }

    println!("aoc_1_2: {}", sum);
}

/* For example, if you see the following box IDs:

abcdef contains no letters that appear exactly two or three times.
bababc contains two a and three b, so it counts for both.
abbcde contains two b, but no letter appears exactly three times.
abcccd contains three c, but no letter appears exactly two times.
aabcdd contains two a and two d, but it only counts once.
abcdee contains two e.
ababab contains three a and three b, but it only counts once.*/
pub fn aoc_2_1() {
    let box_ids = get_input_file_as_string("aoc_2_input.txt");

    let mut exactly_2_num : u32 = 0;
    let mut exactly_3_num : u32 = 0;

    for bar_code in box_ids.lines() {

        let mut map = HashMap::new();

        for letter in bar_code.split("") {
            if letter == "" {
                // `.split` puts a "" at the beginning and end, so we want to skip that
                continue;
            }
            let count = map.get(letter);

            match count {
                Some(count) => map.insert(letter, count + 1),
                None => map.insert(letter, 1),
            };
        }

        let mut three_seen = false;
        let mut two_seen = false;
        for (_, val) in map.iter() {
            if !three_seen && val == &3 {
                three_seen = true;
                exactly_3_num+= 1;
            }
            if !two_seen && val == &2 {
                two_seen = true;
                exactly_2_num+= 1;
            }
            if two_seen && three_seen {
                break;
            }
        }
    }

    let checksum = exactly_2_num * exactly_3_num;
    println!("aoc_2_1: checksum: {}, exactly 2: {}, exactly 3: {}", checksum, exactly_2_num, exactly_3_num);
}

pub fn aoc_2_2() {

}

pub fn aoc_4() {

}

pub fn aoc_5() {

}

pub fn aoc_6() {

}

fn get_input_file_as_string(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn get_input_as_vec(freqs: String) -> Vec<i32> {
    freqs
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}