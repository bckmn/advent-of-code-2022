use std::collections::HashSet;
use std::cmp::{ max, min };

fn main() {
    let input = helpers::read_file("inputs", 4);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|pair| {
            let parsed_pairs = parse_pair(pair);

            let elf1: HashSet<&u8> = parsed_pairs.0.iter().collect();
            let elf2: HashSet<&u8> = parsed_pairs.1.iter().collect();

            elf2.is_subset(&elf1) || elf1.is_subset(&elf2)
        })
        .filter(|x| *x == true)
        .count()
}

pub fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|pair| {
            let a = parse_pair(pair);
            
            max(a.0.first(), a.1.first()) <= 
            min(a.0.last(), a.1.last())
        })
        .count()
}

fn parse_pair(pair: &str) -> (Vec<u8>, Vec<u8>) {
    pair
        .split_once(",")
        .map(|(elf1, elf2)| (parse_elf(elf1), parse_elf(elf2)))
        .unwrap()
}

fn parse_elf(elf: &str) -> Vec<u8> {
    elf.split_once("-")
        .map(|(start, end)| {
            (start.parse::<u8>().unwrap()
            ..
            end.parse::<u8>().unwrap() + 1)
            .collect()
        })
        .unwrap()
}