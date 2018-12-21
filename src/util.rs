pub fn get_input_file_as_string(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn get_input_as_vec(freqs: String) -> Vec<i32> {
    freqs
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}