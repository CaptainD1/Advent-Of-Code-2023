use std::fs;

#[derive(Debug)]
struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let games = contents.lines().filter_map(|line| parse_line(line));

    let sum: usize = games.clone().map(|game| check_valid_game(&game)).sum();

    let power_sum: usize = games.map(|game| min_game_cubes(&game)).sum();

    println!("The answer for part 1 is {}", sum);

    println!("The answer for part 2 is {}", power_sum);
}

fn parse_line(line: &str) -> Option<Game> {
    let id_start = 5;
    let id_end = line[5..].find(':')? + 5;

    let id: usize = line[id_start..id_end].parse().ok()?;

    let pull_strings = line[id_end+1..].split(';');

    let cube_sets: Vec<CubeSet> = pull_strings.map(|pull| parse_pull(pull)).collect();

    Some(Game{id, cube_sets})
}

fn parse_pull(pull: &str) -> CubeSet {
    let mut cube_set = CubeSet {red: 0, green: 0, blue: 0};
    let cubes = pull.split(',');
    for cube_color in cubes {
        let q_index_start = cube_color.find(|ch| char::is_ascii_digit(&ch));
        if let Some(q_index_start) = q_index_start {
            let mut q_index_end = q_index_start;
            for (i, ch) in cube_color[q_index_start..].char_indices() {
                if !ch.is_ascii_digit() {
                    q_index_end += i;
                    break;
                }
            }
            let quantity = cube_color[q_index_start..q_index_end].parse().unwrap();
            if cube_color.contains("red") {
                cube_set.red = quantity;
            } else if cube_color.contains("green") {
                cube_set.green = quantity;
            } else if cube_color.contains("blue") {
                cube_set.blue = quantity;
            }
        }
    }
    cube_set
}

fn check_valid_game(game: &Game) -> usize {
    if game.cube_sets.iter().map(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14).all(|f| f) {
        game.id
    } else {
        0
    }
}

fn min_game_cubes(game: &Game) -> usize {
    let reds = game.cube_sets.iter().map(|game| game.red).max().unwrap();
    let greens = game.cube_sets.iter().map(|game| game.green).max().unwrap();
    let blues = game.cube_sets.iter().map(|game| game.blue).max().unwrap();

    reds * greens * blues
}