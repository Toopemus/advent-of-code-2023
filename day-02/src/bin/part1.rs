use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let sum: i32 = sum_possible_games("games.txt");
    println!("{}", sum);
}

fn get_possible_id(game: &str) -> Option<i32> {
    let bag_contains = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let game_data: Vec<&str> = game.split(':').collect();
    let game_id = game_data[0];
    let game = game_data[1];
    let handfuls = game.split(|c| c == ',' || c == ';');
    for handful in handfuls {
        let hand_data: Vec<&str> = handful.trim().split(' ').collect();
        let amount: i32 = hand_data[0].parse().unwrap();
        let color = hand_data[1];
        for (key, val) in &bag_contains {
            if key == &color && val < &amount { return None }
        }
    }
    let split_game: Vec<&str> = game_id.split(' ').collect();
    let id: i32 = split_game[1].parse().unwrap();

    Some(id)
}

fn sum_possible_games(filename: &str) -> i32 {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        sum = lines
            .filter_map(|line| line.ok())
            .filter_map(|game| get_possible_id(&game))
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
        assert_eq!(get_possible_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Some(1));
        assert_eq!(get_possible_id("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Some(2));
        assert_eq!(get_possible_id("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), None);
        assert_eq!(get_possible_id("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), None);
        assert_eq!(get_possible_id("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Some(5));
    }
}
