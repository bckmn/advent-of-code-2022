use std::env;
use std::fs;

pub fn part_one(input: &str) -> u32 {
    let sorted_calories = sort_input(input);

    *sorted_calories.last().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let sorted_calories = sort_input(input);
    
    sorted_calories.iter().rev().take(3).sum()
}

fn main() {
    let input = read_file("inputs", 1);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

fn sort_input(input: &str) -> Vec<u32> {
    let mut calories_per_elf: Vec<u32> = input
        .split("\n\n")
        .map(|elf| 
            elf.lines().map(|cal|
                cal.parse::<u32>().unwrap())
            .sum())
        .collect();

    calories_per_elf
        .sort_by(|a, b| a.partial_cmp(b)
        .unwrap());

    calories_per_elf
}