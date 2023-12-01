use regex::Regex;
use std::fs;

fn to_digit(s: &str) -> Option<u32> {
    if let Ok(x) = s.parse::<u32>() {
        return Some(x);
    }
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let digit_pattern_str = "one|two|three|four|five|six|seven|eight|nine|[0-9]";
    let first_regex = Regex::new(&format!("({digit_pattern_str}).*")).unwrap();
    let last_regex = Regex::new(&format!(".*({digit_pattern_str})")).unwrap();
    let ans: u32 = input
        .split("\n")
        .map(|line| {
            let maybe_first = first_regex
                .captures_iter(line)
                .next()
                .map(|c| c.extract())
                .and_then(|(_, [s])| to_digit(s));
            let maybe_last = last_regex
                .captures_iter(line)
                .next()
                .map(|c| c.extract())
                .and_then(|(_, [s])| to_digit(s));
            if let (Some(first), Some(last)) = (maybe_first, maybe_last) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .sum();
    println!("{ans}");
}
