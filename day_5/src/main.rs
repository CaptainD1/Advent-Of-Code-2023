use std::collections::HashMap;
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

    let contents = fs::read_to_string(&args.filename);

    if let Ok(contents) = contents {
        if args.timing {
            let now = SystemTime::now();
            for _ in 0..args.loops {
                calculate(&contents, false);
            }
            let duration = (now.elapsed().unwrap().as_nanos() as f64 / args.loops as f64) / 1e6;
            println!("Total run time: {:.4}ms (after {} loops)", duration, args.loops);
        } else {
            let lowest_location = calculate(&contents, args.debug);
            if args.debug {
                println!();
            }
            println!("The lowest location number is: {}", lowest_location);
        }
    } else {
        println!("Error opening file '{}'", &args.filename);
    }
}

fn calculate(contents: &str, debug: bool) -> u64 {

    let mut lines = contents.lines();
    let header = lines.next().unwrap();

    let origin: Vec<_> = header[7..].split_ascii_whitespace().map(|item| item.parse::<u64>().unwrap()).collect();

    let mut link_map = HashMap::new();
    let mut main_map = HashMap::new();

    let mut current_map_id = "soil";
    let mut entry_vec;

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }
        let first_char = line.chars().next().unwrap();
        if first_char.is_alphabetic() { // is map link
            let link: Vec<_> = line.strip_suffix(" map:").unwrap().split("-to-").collect();
            link_map.insert(link[0], link[1]);
            current_map_id = link[1];
        } else if first_char.is_ascii_digit() { // is map entry
            if !main_map.contains_key(&current_map_id) {
                main_map.insert(current_map_id, Vec::new());
            }
            entry_vec = main_map.get_mut(&current_map_id).unwrap();

            let nums: Vec<_> = line.split_ascii_whitespace().map(|item| item.parse::<u64>().unwrap()).collect();
            entry_vec.push((nums[1], nums[0], nums[2]));
        }
    }

    for entry in main_map.values_mut() {
        entry.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    let mut id;
    let mut lowest_id = u64::MAX;

    for start_id in origin {
        let mut map_id = "seed";
        id = start_id;
        if debug {
            println!("\nCurrent seed check: {}", id);
        }
        while let Some(new_map_id) = link_map.get(map_id) {
            map_id = new_map_id;
            if debug {
                println!("Currently converting to: '{}'", map_id);
                print!("{} -> ", id);
            }
            id = connect_chain(&main_map[map_id], id);
            if debug {
                println!("{}", id);
            }
        }
        lowest_id = cmp::min(lowest_id, id);
    }

    lowest_id
}

fn connect_chain(entries: &Vec<(u64, u64, u64)>, id: u64) -> u64 {
    for entry in entries {
        if id >= entry.0 && id < entry.0 + entry.2 {
            let offset = id - entry.0;
            return entry.1 + offset;
        } else if id < entry.0 {
            return id;
        }
    }
    id
}