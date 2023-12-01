use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines("calibration.txt") {
        for line in lines {
            if let Ok(ip) = line {
                sum += get_calibration_value(&ip);
            }
        }
    }
    println!("{}", sum);
}

fn get_calibration_value(line: &str) -> i32 {
    let digit_map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    let mut digit: Option<char> = None;
    let mut result = String::from("");
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            digit = Some(c);
            if result.len() == 0 {
                result.push(c);
            }
        }
        for (key, val) in &digit_map {
            let substring = &line[i..];
            if substring.starts_with(key) {
                digit = Some(*val);
                if result.len() == 0 {
                    result.push(*val);
                }
            }
        }
    }
    match digit {
        Some(d) => result.push(d),
        None => {}
    }

    result.parse().unwrap()
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
    fn get_calibration_value_works() {
        assert_eq!(get_calibration_value("two1nine"), 29);
        assert_eq!(get_calibration_value("eightwothree"), 83);
        assert_eq!(get_calibration_value("abcone2threexyz"), 13);
        assert_eq!(get_calibration_value("xtwone3four"), 24);
        assert_eq!(get_calibration_value("4nineeightseven2"), 42);
        assert_eq!(get_calibration_value("zoneight234"), 14);
        assert_eq!(get_calibration_value("7pqrstsixteen"), 76);
    }
}
