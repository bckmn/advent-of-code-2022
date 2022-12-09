fn main() {
    let input = helpers::read_file("inputs", 2);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

pub fn part_one(input: &str) -> u32 {
    let rounds: Vec<(u32, u32)> = input
    .lines()
    .filter_map(|l| {
        let bytes = l.as_bytes();
        Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
    })
    .collect();

    rounds.iter().map(|r| calculate_score(*r)).sum()
}

pub fn part_two(input: &str) -> u32 {
    let rounds: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|l| {
            let bytes = l.as_bytes();
            Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
        })
        .collect();

    rounds.iter()
        .map(|play| {
            let next_move = match play.1 {
                0 => (play.0 + 2) % 3,
                1 => play.0,
                2 => (play.0 + 1) % 3,
                _ => unreachable!(),
            };
            calculate_score((play.0, next_move))
        })
        .sum()
}

fn calculate_score(round: (u32, u32)) -> u32 {
    let score = (3 - (2 + round.0 - round.1) % 3) % 3 * 3;
    score + round.1 + 1
}