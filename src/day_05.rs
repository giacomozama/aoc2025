use std::cmp::Ordering;

use crate::day::Day;

pub struct Day05;

pub struct Day05Part1Result {
    result: u32,
    merged_ranges: Vec<(u64, u64)>,
}

impl std::fmt::Display for Day05Part1Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.result.fmt(f)
    }
}

impl Day for Day05 {
    type Input = (Vec<(u64, u64)>, Vec<u64>);
    type Output1 = Day05Part1Result;
    type Output2 = u64;

    fn id() -> String {
        "05".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        let mut lines = input.lines();

        let ranges = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.split("-").map(|n| n.parse::<u64>().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect();

        let items = lines.map(|l| l.parse::<u64>().unwrap()).collect();

        (ranges, items)
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (ranges, items) = input;

        let mut sorted_ranges = ranges.to_owned();
        sorted_ranges.sort_by(|(a, b), (c, d)| {
            let order = a.cmp(c);
            match order {
                Ordering::Equal => b.cmp(d),
                _ => order,
            }
        });

        let mut merged_ranges = Vec::<(u64, u64)>::new();
        let (mut cur_start, mut cur_end) = sorted_ranges[0];

        for &(start, end) in sorted_ranges.iter().skip(1) {
            if end <= cur_end {
                continue;
            }
            if start <= cur_end + 1 {
                cur_end = end;
                continue;
            }
            merged_ranges.push((cur_start, cur_end));
            cur_start = start;
            cur_end = end;
        }

        merged_ranges.push((cur_start, cur_end));

        let result = items
            .iter()
            .filter_map(|&item| {
                merged_ranges
                    .binary_search_by(|&(start, end)| {
                        if item < start {
                            Ordering::Greater
                        } else if item > end {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    })
                    .ok()
            })
            .count() as u32;

        Day05Part1Result {
            result,
            merged_ranges,
        }
    }

    fn part_2(_: &Self::Input, part_1_output: Self::Output1) -> Self::Output2 {
        (&part_1_output.merged_ranges)
            .iter()
            .map(|&(start, end)| end - start + 1)
            .sum()
    }
}
