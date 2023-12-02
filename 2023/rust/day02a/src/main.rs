use std::fs;
use std::ops::Add;

struct Cubes(u32, u32, u32);

impl Add for Cubes {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn process_game(game: &str) -> Cubes {
    game.split(", ")
        .map(|s| -> Option<Cubes> {
            let parts: Vec<&str> = s.split(" ").collect();
            let num = parts.get(0)?.trim().parse::<u32>().ok()?;
            match parts.get(1)?.trim() {
                "red" => Some(Cubes(num, 0, 0)),
                "green" => Some(Cubes(0, num, 0)),
                "blue" => Some(Cubes(0, 0, num)),
                _ => None,
            }
        })
        .filter_map(|x| x)
        .fold(Cubes(0, 0, 0), |acc, c| acc + c)
}

fn process_line(line: &str) -> Option<u32> {
    let parts: Vec<&str> = line.split(": ").collect();
    let id = parts.get(0)?.split(" ").nth(1)?.parse::<u32>().ok()?;
    let valid = parts
        .get(1)?
        .split("; ")
        .map(|game| process_game(game))
        .all(|cubes| cubes.0 <= 12 && cubes.1 <= 13 && cubes.2 <= 14);
    if valid {
        Some(id)
    } else {
        None
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let ans: u32 = input
        .split("\n")
        .map(|line| process_line(line))
        .filter_map(|x| x)
        .sum();
    println!("{ans}");
}
