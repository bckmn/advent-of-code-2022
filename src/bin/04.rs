use std::collections::HashSet;

fn main() {
    let input = helpers::read_file("inputs", 4);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

pub fn part_one(input: &str) -> u32 {
    let test = String::from("1-4,3-4");
    let s = parse_pairs(&test);
    println!("test: {:?}", s);
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn parse_pairs(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            let a = parse_pair(pair);
            let elf1: HashSet<&u8> = a[0].iter().collect();
            let elf2: HashSet<&u8> = a[1].iter().collect();
            println!("elf1: {:?}, a0: {:?}", elf1, a[0]);
            println!("elf2: {:?}, a1: {:?}", elf2, a[1]);
            elf2.is_subset(&elf1)
        })
        .collect::<Vec<bool>>()
        .into_iter()
        .filter(|x| *x)
        .count()
}

fn parse_pair(pair: &str) -> Vec<Vec<u8>> {
    pair
        .split(",")
        .map(|elf| parse_elf(elf))
        .collect::<Vec<Vec<u8>>>()
}

fn parse_elf(elf: &str) -> Vec<u8> {
    elf.split("-")
        .map(|section| section.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}