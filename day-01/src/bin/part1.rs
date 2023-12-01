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
    let mut digit: Option<char> = None;
    let mut result = String::from("");
    for c in line.chars() {
        if c.is_digit(10) {
            digit = Some(c);
            if result.len() == 0 {
                result.push(c);
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
        assert_eq!(get_calibration_value("1abc2"), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }
}
