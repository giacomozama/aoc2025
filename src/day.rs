use std::fmt::Display;

pub trait Day {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn id() -> String;
    fn parse_input(input: &str) -> Self::Input;
    fn part_1(input: &Self::Input) -> Self::Output1;
    fn part_2(input: &Self::Input, part_1_output: &Self::Output1) -> Self::Output2;
}
