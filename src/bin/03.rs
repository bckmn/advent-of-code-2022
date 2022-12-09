use itertools::Itertools;

fn main() {
    let input = helpers::read_file("inputs", 3);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

pub fn part_one(input: &str) -> u32 {
    input
            .lines()
            .map(|letters| {
                let item_numbers = convert_to_item_number(&letters);
                let slices = item_numbers.split_at(item_numbers.len() / 2);

                *get_priorities(slices).first().unwrap() as u32
            })
            .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|letters| convert_to_item_number(letters))
        .tuples()
        .map(|(a, b, c)| {
            let priority = get_priorities((&a, &get_priorities((&b, &c))));
            *priority.first().unwrap() as u32
        })
        .sum::<u32>()
}

/* 
    Priority is the value that appears in both of the slices. 
    For input ([1,2,3], [3,4,5]) the priority is 3
*/  
fn get_priorities(slices: (&[u8], &[u8])) -> Vec<u8> {
    slices.0
        .iter()
        .copied()
        .filter(|x| {
            slices.1.contains(x)
        })
        .collect::<Vec<u8>>()
}

/*
    'a' will be item number 1
    'z' will be item number 26
    'A' will be item number 27
    'Z' will be item number 52
 */
fn convert_to_item_number(letters: &str) -> Vec<u8> {
    letters
        .as_bytes()
        .iter()
        .map(|&x| {
            if x <= 90 {
                x - 38
            }
            else {
                x - 96
            }
        })
        .collect()
}