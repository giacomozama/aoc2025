use crate::day::Day;

pub struct Day03;

impl Day for Day03 {
    type Input = Vec<Vec<u8>>;
    type Output1 = u64;
    type Output2 = u64;

    fn id() -> String {
        "03".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        solve_for(input, 2)
    }

    fn part_2(input: &Self::Input, _: Self::Output1) -> Self::Output2 {
        solve_for(input, 12)
    }
}

fn solve_for(input: &<Day03 as Day>::Input, window_size: usize) -> u64 {
    let mut res = 0u64;

    for bank in input {
        let mut digits = bank[bank.len() - window_size..].to_owned();

        for &b in bank.iter().rev().skip(window_size) {
            if b < digits[0] {
                continue;
            }

            let mut prev = digits[0];
            digits[0] = b;

            for i in 1..window_size {
                if digits[i] > prev {
                    break;
                }

                std::mem::swap(&mut prev, &mut digits[i]);
            }
        }

        res += digits.iter().fold(0, |acc, &d| acc * 10 + d as u64);
    }

    res
}
