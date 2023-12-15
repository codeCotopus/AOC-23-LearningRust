use std::collections::HashMap;
use std::str::Split;
use std::usize;

fn main() {
    let input = include_str!("input1.txt");
    let output = part2(input);
    dbg!(output);

    let sove = solve(input);
    dbg!(sove)

}



#[derive(Debug, PartialEq)]
struct NumberPosition {
    number: u32,
    position: usize,
}

/// Gets the position of the digits in the input string
/// ## Examples
/// - "1a2b3c" -> NumberPosition { number: 1, position: 0 }, NumberPosition { number: 2, position: 2 }, NumberPosition { number: 3, position: 4 }
fn find_digits(input: &str) -> Vec<NumberPosition> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| NumberPosition {
            number: c.to_digit(10).unwrap(),
            position: i,
        })
        .collect()
}



/// Gets the position of the first character of spelled numbers in the input string
/// ## Examples
/// - "onediaosd" -> NumberPosition { number: 1, position: 0 }
/// - "aidotwo" -> NumberPosition { number: 2, position: 4 }
fn find_spelled_numbers(input: &str) -> Vec<NumberPosition> {
    let spelled_numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut positions: Vec<NumberPosition> = Vec::new();
    for spelled_number in spelled_numbers.clone() {
        let mut start = 0;
        while let Some(position) = input[start..].find(spelled_number) {
            positions.push(NumberPosition {
                number: spelled_numbers.iter().position(|&n| n == spelled_number).unwrap() as u32 + 1,
                position: start + position,
            });
            start += position + spelled_number.len();
        }
    }

    // sort the positions by position
    positions.sort_by_key(|np| np.position);
    positions
}

pub fn solve(input: &str) {
    // iterate over each line and get the two digits in the string
    let ans = input.lines().map(|line| get_numbers_from_line(line)).sum::<u32>();

    println!("Answer: {}", ans)
}

fn get_numbers_from_line(line: &str) -> u32 {
    // find all the numbers in the input
    // get only the first NumberPosition and last by position
    // sum the numbers
    // print the sum
    let digits = find_digits(line);
    let spelled_numbers = find_spelled_numbers(line);
    let mut numbers: Vec<NumberPosition> = digits;
    numbers.extend(spelled_numbers);
    numbers.sort_by_key(|np| np.position);


    // get the first and last number
    let first = numbers.first().expect("there should be at least one number");
    let last = numbers.last().expect("there should be at least one number");
    let answer = first.number * 10 + last.number;

    println!("{} -> {:?} + {:?} = {}", line, first, last, answer);

    // sum the numbers
    answer
}

fn part2(input: &str) -> String {
    let parts = input.split("\n");
    let numbers = get_values(parts.clone());
    let s: i64 = numbers.iter().sum();
    s.to_string()
}

fn get_values(parts: Split<&str>) -> Vec<i64> {
    // Create a map of strings to int
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    //ARray to store values
    let mut values: Vec<i64> = Vec::new();
    for part in parts {
        dbg!(part);
        let first_char_index = get_first_digit(part, map.clone()).unwrap();
        let second_char_index = get_second_digit(part, map.clone()).unwrap();
        let digit_as_string = first_char_index + &second_char_index;
        dbg!(digit_as_string.clone());
        values.push(digit_as_string.parse().unwrap());
    }
    return values;
}

fn get_first_digit(part: &str, map: HashMap<&str, i32>) -> Option<String> {
    let textIndex = find_first_key_index(&map.clone(), part);
    let first_number_index = get_first_number_digit_position(part);

    let mut first;
    if textIndex < first_number_index {
        first = find_first_string_character(&map, part);
    } else {
        first = part.chars().nth(first_number_index).unwrap().to_string().parse::<i32>().unwrap();
    }
    Some(first.to_string())
}

fn get_second_digit(part: &str, map: HashMap<&str, i32>) -> Option<String> {
    let textIndex = find_last_key_index(&map.clone(), part);
    let first_number_index = get_last_number_digit_position(part);

    let mut second;
    if textIndex > first_number_index {
        second = find_last_string_character(&map, part);
    } else {
        second = part.chars().nth(first_number_index).unwrap().to_string().parse::<i32>().unwrap();
    }
    Some(second.to_string())
}

fn get_first_number_digit_position(part: &str) -> usize {
    for c in part.chars() {
        if c.is_digit(10) {
            return part.find(c).unwrap();
        }
    }
    usize::MAX
}

fn get_last_number_digit_position(part: &str) -> usize {
    for c in part.chars().rev() {
        if c.is_digit(10) {
            return part.find(c).unwrap();
        }
    }
    usize::MIN
}

fn get_last__number_digit(part: &str) -> usize {
    for c in part.chars().rev() {
        if c.is_digit(10) {
            return part.rfind(c).unwrap();
        }
    }
    usize::MIN
}

fn find_first_key_index(map: &HashMap<&str, i32>, input_string: &str) -> usize {
    let mut min_index: usize = usize::MAX;
    for key in map.keys() {
        if let Some(index) = input_string.find(key) {
            if index < min_index {
                min_index = index;
            }
        }
    }
    return min_index;
}

fn find_last_key_index(map: &HashMap<&str, i32>, input_string: &str) -> usize {
    let mut last_index: usize = usize::MIN;
    for key in map.keys() {
        if let Some(index) = input_string.rfind(key) {
            if index > last_index {
                last_index = index;
            }
        }
    }
    last_index
}


fn find_first_string_character(map: &HashMap<&str, i32>, input_string: &str) -> i32 {
    let mut min_index: usize = usize::MAX;
    let mut digit: i32 = i32::MAX;
    for key in map.keys() {
        if let Some(index) = input_string.find(key) {
            if index < min_index {
                min_index = index;
                digit = *map.get(key).unwrap();
            }
        }
    }
    digit
}

fn find_last_string_character(map: &HashMap<&str, i32>, input_string: &str) -> i32 {
    let mut max_index: usize = usize::MIN;
    let mut digit: i32 = i32::MIN;
    for key in map.keys() {
        if let Some(index) = input_string.rfind(key) {
            if index > max_index {
                max_index = index;
                digit = *map.get(key).unwrap();
            }
        }
    }
    digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");

        assert_eq!(result, "281".to_string());
    }
}