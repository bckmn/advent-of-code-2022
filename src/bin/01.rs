fn main() {
    let input = helpers::read_file("inputs", 1);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

pub fn part_one(input: &str) -> u32 {
    let sorted_calories = sort_input(input);

    *sorted_calories.last().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let sorted_calories = sort_input(input);
    
    sorted_calories.iter().rev().take(3).sum()
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