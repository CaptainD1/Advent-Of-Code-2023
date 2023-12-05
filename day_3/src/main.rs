use std::str::from_utf8;
use std::{fs, cmp};
use std::time::SystemTime;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename
    #[arg(short, long, default_value_t = String::from("input.txt"))]
    filename: String,

    /// Whether or not to time the program execusion
    #[arg(short, long, default_value_t = false)]
    timing: bool,

    /// How many loops to run when measuring timing
    #[arg(short, long, default_value_t = 1000)]
    loops: usize,

    /// Display debug info
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

fn main() {

    let args: Args = Args::parse();

    let contents = fs::read(&args.filename);

    if let Ok(contents) = contents {
        if args.timing {
            let now = SystemTime::now();
            for _ in 0..args.loops {
                calculate(&contents, false);
            }
            let duration = (now.elapsed().unwrap().as_nanos() as f64 / args.loops as f64) / 1e6;
            println!("Total run time: {:.4}ms (after {} loops)", duration, args.loops);
        } else {
            let (part_sum, gear_sum) = calculate(&contents, args.debug);
            if args.debug {
                println!();
            }
            println!("The part sum is: {}", part_sum);
            println!("The gear sum is: {}", gear_sum);
        }
    } else {
        println!("Error opening file '{}'", &args.filename);
    }
}

fn calculate(contents: &Vec<u8>, debug: bool) -> (usize, usize) {
    let lines: Vec<&[u8]> = contents.split(|num| *num == b'\n').filter(|line| line.len() > 1).collect();

    let mut part_sum = 0;
    let mut gear_sum = 0;

    let mut total = 0;
    
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            // Is a part
            if !is_digit(*ch) && !(*ch == b'.') {
                let nums = get_part_nums(&lines, x, y, debug);
                total += nums.len();
                part_sum += nums.iter().sum::<usize>();
                if *ch == b'*' && nums.len() == 2 {
                    gear_sum += nums[0] * nums[1];
                }
            }
        }
    }
    if debug {
        println!("Total part numbers found: {}", total);
    }
    (part_sum, gear_sum)
}

fn get_part_nums(lines: &Vec<&[u8]>, x_pos: usize, y_pos: usize, debug: bool) -> Vec<usize> {
    let x_start = x_pos.checked_sub(1).unwrap_or(0);
    let x_end = cmp::min(x_pos + 2, lines[0].len());
    let y_start = y_pos.checked_sub(1).unwrap_or(0);
    let y_end = cmp::min(y_pos + 2, lines.len());

    let mut in_num = false;
    let mut nums: Vec<usize> = Vec::new();

    if debug {
        println!("\nCenter pos: ({}, {})", x_pos, y_pos);
        println!("{}", lines[y_start..y_end].iter().map(|line| std::str::from_utf8(&line[x_start..x_end]).unwrap()).collect::<Vec<&str>>().join("\n"));
    }

    for y in y_start..y_end {
        for x in x_start..x_end {
            let ch = lines[y][x];
            if is_digit(ch){
                if !in_num {
                    in_num = true;
                    nums.push(get_full_number(lines, x, y, debug));
                }
            } else if in_num { // Was searching through a number but no longer in one
                in_num = false;
            }
        }
        in_num = false;
    }
    nums
}

fn get_full_number(lines: &Vec<&[u8]>, x_pos: usize, y_pos: usize, debug: bool) -> usize {
    let mut x = x_pos.checked_sub(1).unwrap_or(0);
    while is_digit(lines[y_pos][x]) && x > 0 {
        x -= 1;
    }
    // Prevent issues with negative usize
    let x_start = if is_digit(lines[y_pos][x]) {
        x
    } else {
        x + 1
    };
    x = x_pos + 1;
    while x < lines[y_pos].len() && is_digit(lines[y_pos][x]) {
        x += 1;
    }
    let num = from_utf8(&lines[y_pos][x_start..x]).unwrap().parse().unwrap();
    if debug {
        println!("{} @ ({}, {}) -> ({}, {})", num, x_start, y_pos, x, y_pos);
    }
    num
}

fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}