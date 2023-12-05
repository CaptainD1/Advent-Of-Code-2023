use std::fs;
use std::time::SystemTime;

const WORD_NUMS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {

    let times = 1000;

    let now = SystemTime::now();
    for _ in 0..times {
        let contents = fs::read_to_string("input.txt").unwrap();
        calculate(&contents);
    }
    let duration = (now.elapsed().unwrap().as_nanos() as f64 / times as f64) / 1e6;

    println!("Total run time: {}ms", duration);

    let contents = fs::read_to_string("input.txt").unwrap();
    let sum = calculate(&contents);
    println!("The total is {}", sum);
}

fn calculate(contents: &str) -> u32 {

    contents.lines().map(|line| {
        let mut first_num = None;
        let mut last_num = None;
        for (i, ch) in line.char_indices() {
            first_num = get_digit(line, i, ch);
            if first_num.is_some() {
                break;
            }
        }
        for (i, ch) in line.char_indices().rev() {
            last_num = get_digit(line, i, ch);
            if last_num.is_some() {
                break;
            }
        }
        if let (Some(first_num), Some(last_num)) = (first_num, last_num) {
            (first_num + &last_num).parse::<u32>().unwrap()
        } else {
            0
        }
    }).sum()
}

fn get_digit(line: &str, index: usize, ch: char) -> Option<String> {
    if ch.is_ascii_digit() {
        return Some(ch.to_string());
    }
    for (i, word) in WORD_NUMS.iter().enumerate() {
        if line[index..].starts_with(word) {
            return Some((i + 1).to_string());
        }
    }
    None
}