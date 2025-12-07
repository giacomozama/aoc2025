use std::{
    collections::{HashSet, VecDeque},
    iter,
};

use crate::day::Day;

pub struct Day07;

impl Day for Day07 {
    type Input = (usize, Vec<Vec<bool>>);
    type Output1 = u32;
    type Output2 = u64;

    fn id() -> String {
        "07".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        let board = input
            .lines()
            .enumerate()
            .filter(|&(i, _)| i & 1 == 0)
            .map(|(_, l)| l.chars().map(|c| c == '^').collect())
            .collect();

        let start = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap();

        (start, board)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (start, board) = input;
        let mut positions = VecDeque::from([*start]);
        let mut result = 0;

        for y in 1..board.len() {
            let mut visited = HashSet::<_>::new();
            for _ in 0..positions.len() {
                let pos = positions.pop_front().unwrap();
                if board[y][pos] {
                    result += 1;
                    if visited.insert(pos - 1) {
                        positions.push_back(pos - 1);
                    }
                    if visited.insert(pos + 1) {
                        positions.push_back(pos + 1);
                    }
                } else if visited.insert(pos) {
                    positions.push_back(pos);
                }
            }
        }

        result
    }

    fn part_2(input: &Self::Input, _: &Self::Output1) -> Self::Output2 {
        let (start, board) = input;

        let mut dp: Vec<u64> = iter::repeat(0).take(board[0].len()).collect();
        dp[*start] = 1;

        for y in 1..board.len() {
            for x in 0..board[0].len() {
                if !board[y][x] {
                    continue;
                }
                if x > 0 {
                    dp[x - 1] += dp[x]
                }
                if x < board[0].len() - 1 {
                    dp[x + 1] += dp[x]
                }
                dp[x] = 0
            }
        }

        dp.iter().sum()
    }
}
