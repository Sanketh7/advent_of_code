use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let ans: u32 = input
        .split("\n")
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                Some(first * 10 + last)
            } else {
                None
            }
        })
        .flat_map(|x| x)
        .sum();
    println!("{ans}");
}
