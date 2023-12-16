use regex::{Captures, Regex};

fn main() {
    let input = include_str!("input2.txt");
    let result1 = part1(input);
    println!("Day 2 - Part 1: {result1}");
    let result2 = part2(input);
    println!("Day 2 - Part 1: {result2}");
}

const MAX_BLUE: i32 = 14;
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut result: i32 = 0;

    lines.for_each(|l| {
        let day = get_day_number(l);

        let blue_re = "(\\d+) blue.*?";
        let red_re = "(\\d+) red.*?";
        let green_re = "(\\d+) green.*?";

        let valid_blue = is_day_valid(l, blue_re, MAX_BLUE);
        let valid_red = is_day_valid(l, red_re, MAX_RED);
        let valid_green = is_day_valid(l, green_re, MAX_GREEN);

        if valid_blue && valid_green && valid_red { result = result + day }
    });

    return result.to_string();
}

fn is_day_valid(l: &str, re: &str, max: i32) -> bool {
    let mut valid = true;
    let regex = Regex::new(re).unwrap();
    let iterator = regex.captures_iter(l);
    iterator.for_each(|entry| {
        let total_balls_seen = get_count_from_match(&entry);
        if total_balls_seen > max {
            valid = false;
        }
    });
    return valid;
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut result: i32 = 0;

    lines.for_each(|l| {
        let min_blue = get_minimum_from_line(l, "(\\d+) blue.*?");
        let min_red: i32 = get_minimum_from_line(l, "(\\d+) red.*?");
        let min_green: i32 = get_minimum_from_line(l, "(\\d+) green.*?");

        result = result + (min_red * min_green * min_blue);
    });

    result.to_string()
}

fn get_minimum_from_line(l: &str, re: &str) -> i32 {
    let mut min: i32 = 0;
    let regex = Regex::new(re).unwrap();
    let iterator = regex.captures_iter(l);
    iterator.for_each(|entry| {
        let total_balls_seen = get_count_from_match(&entry);
        if total_balls_seen > min {
            min = total_balls_seen;
        }
    });
    min
}

fn get_count_from_match(entry: &Captures) -> i32 {
    entry[1].parse::<i32>().unwrap()
}

fn get_day_number(l: &str) -> i32 {
    let game_day_regex = Regex::new("Game (\\d+)").unwrap();
    let Some(caps) = game_day_regex.captures(l) else { panic!("can't parse day") };
    let day = &caps[1].parse::<i32>();
    return match day {
        Ok(d) => { *d }
        Err(_) => { 0 }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA_FOR_TESTS: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_test() {
        let solution: i32 = part1(SAMPLE_DATA_FOR_TESTS).parse().unwrap();
        assert_eq!(solution, 8);
    }

    #[test]
    fn part2_test() {
        let solution: i32 = part2(SAMPLE_DATA_FOR_TESTS).parse().unwrap();
        assert_eq!(solution, 2286);
    }
}