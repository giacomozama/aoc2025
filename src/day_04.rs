use std::collections::VecDeque;

use crate::day::Day;

pub struct Day04;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

impl Day for Day04 {
    type Input = Vec<Vec<bool>>;
    type Output1 = u32;
    type Output2 = u32;

    fn id() -> String {
        "04".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| l.chars().map(|c| c == '@').collect())
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result = 0u32;

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] && can_roll_be_accessed(input, y, x) {
                    result += 1;
                }
            }
        }

        result
    }

    fn part_2(input: &Self::Input, _: Self::Output1) -> Self::Output2 {
        let mut queue = VecDeque::<(usize, usize)>::new();

        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] {
                    queue.push_front((y, x));
                }
            }
        }

        let mut result = 0u32;
        let mut state = input.to_owned();

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (y, x) = queue.pop_back().unwrap();
                if !state[y][x] || !can_roll_be_accessed(&state, y, x) {
                    continue;
                }
                result += 1;
                state[y][x] = false;
                for nb in get_roll_neighbors(&state, y, x) {
                    let (y, x) = nb;
                    if state[y][x] {
                        queue.push_front(nb);
                    }
                }
            }
        }

        result
    }
}

fn get_roll_neighbors(
    input: &<Day04 as Day>::Input,
    y: usize,
    x: usize,
) -> impl Iterator<Item = (usize, usize)> {
    DIRECTIONS.iter().filter_map(move |(dy, dx)| {
        let ty = y as i32 + dy;
        let tx = x as i32 + dx;
        if ty < 0 || ty >= input.len() as i32 || tx < 0 || tx >= input[0].len() as i32 {
            None
        } else {
            Some((ty as usize, tx as usize))
        }
    })
}

fn can_roll_be_accessed(input: &<Day04 as Day>::Input, y: usize, x: usize) -> bool {
    let mut adj = 0;
    for (y, x) in get_roll_neighbors(input, y, x) {
        if !input[y][x] {
            continue;
        }
        adj += 1;
        if adj == 4 {
            return false;
        }
    }
    true
}
