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

    let mut exactly_2_num: u32 = 0;
    let mut exactly_3_num: u32 = 0;

    for bar_code in box_ids.lines() {
        // create a map of chars to their number of ocurrences within a bar code
        let mut counts = HashMap::new();
        for letter in bar_code.chars() {
            let counter = counts.entry(letter).or_insert(0);
            *counter += 1;
        }

        // check for occurrences of exactly 2 or 3
        let counts = counts.values().collect::<HashSet<_>>();
        if counts.contains(&2) {
            exactly_2_num += 1;
        }
        if counts.contains(&3) {
            exactly_3_num += 1;
        }
    }

    let checksum = exactly_2_num * exactly_3_num;
    println!(
        "aoc_2_1: checksum: {}, exactly 2: {}, exactly 3: {}",
        checksum, exactly_2_num, exactly_3_num
    );
}

pub fn aoc_2_2() {
    let box_ids = get_input_file_as_string("aoc_2_input.txt");

    println!("{}", aoc_2_2_internal(&box_ids));
}

fn aoc_2_2_internal(box_ids: &str) -> String {
    let mut prototype_fabric_id = "".to_string();
    'outer: for (i, line) in box_ids.lines().enumerate() {
        for box_id in box_ids.lines().skip(i) {
            if check_for_exactly_1_difference(box_id, line) {
                let (lhs, rhs) = split_str_at_difference(&box_id, &line);
                prototype_fabric_id = lhs + &rhs;
                break 'outer;
            }
        }
    }
    if prototype_fabric_id.is_empty() {
        panic!("no match found!");
    }
    prototype_fabric_id.to_string()
}

/// given 2 box ids, return true if there is exactly 1 character difference between them, otherwise
/// return false
fn check_for_exactly_1_difference(box_id: &str, other_box_id: &str) -> bool {
    let zipped_id_iter = box_id.chars().zip(other_box_id.chars());

    let mut exactly_1_char_difference = false;
    for (box_id_char, other_box_id_char) in zipped_id_iter {
        if box_id_char != other_box_id_char {
            // we found a difference in characters!
            if exactly_1_char_difference {
                // but... this is the second difference we found
                exactly_1_char_difference = false;
                break;
            }
            exactly_1_char_difference = true;
        }
    }
    exactly_1_char_difference
}

/// Given 2 &str's that differ by 1 character, return two Strings on the left hand and right hand
/// sides of the splitpoint
fn split_str_at_difference(str : &str, other_str : &str) -> (String, String) {
    let mut zipped_chars = str.chars().zip(other_str.chars());
    let mismatch_index = zipped_chars.position(|(x, y)| x != y).unwrap();
    let (lhs, rhs) = str.split_at(mismatch_index);

    (lhs.to_string(), rhs.get(1..rhs.len()).unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_aoc_2_2() {
        let input = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"
        .to_string();
        assert_eq!(aoc_2_2_internal(&input), "fgij".to_string());
    }

    #[test]
    fn assert_exactly_1_char_difference() {
        let str1 = "blah";
        let str2 = "blaf";

        assert_eq!(true, check_for_exactly_1_difference(str1, str2));

        let str1 = "blah";
        let str2 = "blah";
        assert_eq!(false, check_for_exactly_1_difference(str1, str2));

        let str1 = "a";
        let str2 = "ab";
        assert_eq!(false, check_for_exactly_1_difference(str1, str2));
    }

    #[test]
    fn assert_split_str_at_difference() {
        let str1 = "blah";
        let str2 = "byah";

        assert_eq!(("b".to_string(), "ah".to_string()), split_str_at_difference(str1, str2));
    }
}
