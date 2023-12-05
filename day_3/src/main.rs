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
    #[arg(long, default_value_t = 1000)]
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
            println!("The answer is: {}", calculate(&contents, args.debug));
        }
    } else {
        println!("Error opening file '{}'", &args.filename);
    }
}

fn calculate(contents: &Vec<u8>, debug: bool) -> u32 {
    let lines: Vec<&[u8]> = contents.split(|num| *num == b'\n').filter(|line| line.len() > 1).collect();
    let mut start_num = None;
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            // Is a digit
            if *ch >= b'0' && *ch <= b'9' {
                if start_num.is_none() {
                    start_num = Some((x, y));
                }
            } else {
                if let Some(start_num) = start_num.take() {
                    if let Some(part_id) = get_part_num(&lines, start_num, (x, y), debug) {
                        sum += part_id;
                    }
                }
            }
        }
        if let Some(start_num) = start_num.take() {
            if let Some(part_id) = get_part_num(&lines, start_num, (lines[y].len(), y), debug) {
                sum += part_id;
            }
        }
    }
    sum
}

fn get_part_num(lines: &Vec<&[u8]>, start: (usize, usize), end: (usize, usize), debug: bool) -> Option<u32> {
    let start_corner = (start.0.checked_sub(1).unwrap_or(0),
            start.1.checked_sub(1).unwrap_or(0));
            // Assuming all lines are of equal length (which they are)
    let end_corner = (cmp::min(end.0 + 1, lines.len()),
            cmp::min(end.1 + 2, lines.len()));

    let mut is_part_num = false;

    'outer: for y in start_corner.1..end_corner.1 {
        for x in start_corner.0..cmp::min(end_corner.0, lines[y].len()) {
            match lines[y][x] {
                b'0'..=b'9' | b'.' => {},
                _ => {
                    is_part_num = true;
                    break 'outer;
                }
            }
        }
    }
    if debug {
        println!("Number cutout: ({}, {}) -> ({}, {})", start_corner.0, start_corner.1, end_corner.0, end_corner.1);
        println!("Is part num: {}", is_part_num);
        println!("Number: {}", std::str::from_utf8(&lines[start.1][start.0..end.0]).unwrap());
        println!("{}\n", lines[start_corner.1..end_corner.1].iter().map(|line| std::str::from_utf8(&line[start_corner.0..end_corner.0]).unwrap()).collect::<Vec<&str>>().join("\n"));
    }
    if is_part_num {
        Some(std::str::from_utf8(&lines[start.1][start.0..end.0]).unwrap().parse().unwrap())
    } else {
        None
    }
}