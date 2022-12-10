fn main() {
    let input = helpers::read_file("inputs", 5);
    let (starting_grid, moves) = parse_input(input);

    println!("Part 1: {:?}", part_one(starting_grid, moves));
    // println!("Part 2: {}", part_two(&input));
}

fn part_one(starting_grid: Vec<CrateStack>, moves: Vec<(usize, usize)>) -> Vec<char> {
    rearrange_crates(starting_grid, moves)
}

// fn part_two(input: &str) -> usize {
//     0
// }

fn parse_input(input: String) -> (Vec<CrateStack>, Vec<(usize, usize)>) {
    let parts: (&str, &str) = input
        .split_once("\n\n")
        .unwrap();

    (get_starting_grid(parts.0.to_owned()), parse_moves(parts.1.to_owned()))
}

fn get_starting_grid(input: String) -> Vec<CrateStack> {
    let mut rows: Vec<&str> = input
        .lines()
        .collect();
    
    let column_count = rows.pop().unwrap()
        .split_whitespace().last().unwrap()
        .parse::<usize>().unwrap();
    
    let mut stacks: Vec<CrateStack> = Vec::new();
    for _ in 1..column_count+1 {
        stacks.push(CrateStack { crates: Vec::new() });
    }

    rows.reverse();

    for row in rows {
        for (i, ch) in row.chars().enumerate() {
            if (i == 0 || (i - 1) % 4 == 0) && ch != ' ' && ch !='[' {
                stacks[i / 4].crates.push(ch);
            }
        }
    }
    
    stacks
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

fn rearrange_crates(stacks: Vec<CrateStack>, moves: Vec<(usize, usize)>) -> Vec<char> {
    let mut stacks = stacks;
    
    for _move in moves {
        let (from, to) = _move;
        let _crate = stacks[from].pop();
        stacks[to].push(_crate);
    }

    stacks.into_iter().map(|x| *x.crates.last().unwrap()).collect()
}

struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {

    fn push(&mut self, _crate: char) {
        self.crates.push(_crate);
    }

    fn pop(&mut self) -> char {
        self.crates.pop().unwrap()
    }
}