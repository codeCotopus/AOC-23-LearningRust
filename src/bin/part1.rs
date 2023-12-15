use std::str::Split;

fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let parts = input.split("\n");
    let numbers = get_values(parts.clone());
    let s:i64  = numbers.iter().sum();
    s.to_string()
}

fn get_values(parts: Split<&str>) -> Vec<i64> {
    let mut values : Vec<i64> = Vec ::new();
    for part in parts {
        let first = get_first_digit(part).unwrap();
        let second = get_last_digit(part).unwrap();
        let concatenated = first+ &*second;
        let number = concatenated.parse::<i64>().unwrap();
        values.push(number);
    }
    return values;
}

fn get_first_digit(part: &str) -> Option<String> {
    for c in part.chars() {
        if c.is_digit(10) {
            return Some (c.to_string());
        }
    }
    None
}

fn get_last_digit(part: &str) -> Option<String> {
    for c in part.chars().rev() {
        if c.is_digit(10) {
            return Some(c.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142".to_string());
    }
}