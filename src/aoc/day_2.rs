use super::util::get_input_file_as_string;
use std::collections::HashMap;
use std::collections::HashSet;

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
        // create a map of chars to their number of ocurrences within a bar code
        let mut counts = HashMap::new();
        for letter in bar_code.chars() {
            let counter = counts.entry(letter).or_insert(0);
            *counter+= 1;
        }

        // check for occurrences of exactly 2 or 3
        let counts = counts.values().collect::<HashSet<_>>();
        if counts.contains(&2) {
            exactly_2_num+= 1;
        }
        if counts.contains(&3) {
            exactly_3_num+= 1;
        }
    }

    let checksum = exactly_2_num * exactly_3_num;
    println!("aoc_2_1: checksum: {}, exactly 2: {}, exactly 3: {}", checksum, exactly_2_num, exactly_3_num);
}