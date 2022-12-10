use std::array;

type Stacks = [Vec<char>; 9];

fn main() {
    let input = helpers::read_file("inputs", 5);
    let (starting_grid, moves) = parse_input(input);

    println!("Part 1: {}", part_one(starting_grid, moves));

    let input = helpers::read_file("inputs", 5);
    let (starting_grid, moves) = parse_input(input);
    println!("Part 2: {}", part_two(starting_grid, moves));
}

fn part_one(starting_grid: Stacks, moves: Vec<(usize, usize)>) -> String {
    let top_crates = rearrange_crates(starting_grid, moves);

    top_crates
        .iter()
        .map(|x| x.last().unwrap())
        .collect()
}

fn part_two(starting_grid: Stacks, moves: Vec<(usize, usize)>) -> String {
    let top_crates = rearrange_crates(starting_grid, moves);

    top_crates
        .iter()
        .map(|x| x.last().unwrap())
        .collect()
}

fn parse_input(input: String) -> (Stacks, Vec<(usize, usize)>) {
    let parts: (&str, &str) = input
        .split_once("\n\n")
        .unwrap();

    (get_starting_grid(parts.0.to_owned()),
     parse_moves(parts.1.to_owned()))
}

fn get_starting_grid(input: String) -> Stacks {
    array::from_fn(|i| {
        input
            .lines()
            .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<char>>())
            .rev()
            .skip(1)
            .map(|row| row[i])
            .take_while(|&c| !c.is_whitespace())
            .collect::<Vec<char>>()
    })
}

fn parse_moves(moves: String) -> Vec<(usize, usize)> {
    moves
        .lines()
        .map(parse_move_line)
        .into_iter()
        .flatten()
        .collect()
}

fn parse_move_line(_line: &str) -> Vec<(usize, usize)> {
    let parsed: Vec<_> =_line.split_whitespace()
        .collect();

    let count = parsed[1].parse::<usize>().unwrap();
    let from = parsed[3].parse::<usize>().unwrap();
    let to = parsed[5].parse::<usize>().unwrap();

    vec![(from - 1, to - 1); count]
}

fn rearrange_crates(stacks: Stacks, moves: Vec<(usize, usize)>) -> Stacks {
    let mut stacks = stacks;
    
    for _move in moves {
        let (from, to) = _move;
        let _crate = stacks[from].pop().unwrap();
        stacks[to].push(_crate);
    }

    stacks
}