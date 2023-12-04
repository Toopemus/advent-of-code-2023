use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let sum: i32 = sum_powers("games.txt");
    println!("{}", sum);
}

fn get_cubes_power(game: &str) -> i32 {
    let mut cube_minimums = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ]);
    let game_data: Vec<&str> = game.split(':').collect();
    let game = game_data[1];
    let handfuls = game.split(|c| c == ',' || c == ';');
    for handful in handfuls {
        let hand_data: Vec<&str> = handful.trim().split(' ').collect();
        let count: i32 = hand_data[0].parse().unwrap();
        let minimum = cube_minimums.get_mut(hand_data[1]).unwrap();
        if *minimum < count {
            *minimum = count
        }
    }
    let power = cube_minimums.values().fold(1, |acc, x| acc * x);

    power
}

fn sum_powers(filename: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        sum = lines
            .filter_map(|line| line.ok())
            .map(|game| get_cubes_power(&game))
            .sum();
    }
    sum
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_possible_id_works() {
        assert_eq!(get_cubes_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(get_cubes_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(get_cubes_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(get_cubes_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(get_cubes_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
}
