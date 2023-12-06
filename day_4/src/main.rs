use std::collections::BTreeSet;
use std::fs;
use std::str::from_utf8;
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

const NUM_WINNING: usize = 10;
const NUM_OWNED: usize = 25;
const WINNING_START: usize = 10;
const OWNED_START: usize = 42;
const NUM_WIDTH: usize = 3;

struct Card {
    winning_nums: BTreeSet<u8>,
    owned_nums: BTreeSet<u8>
}

impl Card {
    fn from_line(line: &[u8]) -> Card {

        // Creating the arrays and converting to BTrees is faster than just inserting into BTrees
        let mut winning_nums = [0u8; NUM_WINNING];
        let mut owned_nums = [0u8; NUM_OWNED];

        for i in 0..NUM_WINNING {
            winning_nums[i] = from_utf8(&line[WINNING_START + i*NUM_WIDTH..WINNING_START + i*NUM_WIDTH + NUM_WIDTH-1]).unwrap().trim().parse().unwrap();
        }

        for i in 0..NUM_OWNED {
            owned_nums[i] = from_utf8(&line[OWNED_START + i*NUM_WIDTH..OWNED_START + i*NUM_WIDTH + NUM_WIDTH-1]).unwrap().trim().parse().unwrap();
        }

        Card {winning_nums: BTreeSet::from(winning_nums), owned_nums: BTreeSet::from(owned_nums)}
    }
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
            let (point_sum, card_total) = calculate(&contents, args.debug);
            if args.debug {
                println!();
            }
            println!("The point total is: {}", point_sum);
            println!("The card total is: {}", card_total);
        }
    } else {
        println!("Error opening file '{}'", &args.filename);
    }
}

fn calculate(contents: &Vec<u8>, _debug: bool) -> (usize, usize) {
    let cards: Vec<Card> = contents.split(|num| *num == b'\n').filter(|line| line.len() > 1).map(|line| Card::from_line(line)).collect();

    // Find total number of wins for each card
    let num_winning: Vec<_> = cards.iter().map(|card| card.owned_nums.intersection(&card.winning_nums).count()).collect();

    // Add up the points calculated for part 1
    let points = num_winning.iter().filter(|n| **n > 0).map(|n| 2usize.pow(*n as u32 - 1)).sum();
    
    // Count up the cards for part 2
    let mut total_cards = Vec::with_capacity(cards.len());
    total_cards.resize(cards.len(), 1); // Start with 1 of each card
    for (i, win_count) in num_winning.iter().enumerate() {
        for j in i+1..i+win_count+1 { // Apply to next n cards where n is number of wins
            total_cards[j] += total_cards[i] // Add the quantity of the current card
        }
    }

    (points, total_cards.iter().sum())
}