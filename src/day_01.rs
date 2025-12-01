use crate::day::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<i16>;
    type Output1 = u16;
    type Output2 = u16;

    fn id() -> String {
        "01".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let direction = match &l[0..1] {
                    "L" => -1,
                    "R" => 1,
                    _ => panic!("Invalid rotation direction"),
                };

                let distance: i16 = l[1..].parse().expect("Invalid rotation distance");

                distance * direction
            })
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut position = 50i16;
        let mut result = 0u16;

        for &rotation in input {
            position = (position + rotation).rem_euclid(100);
            let sign = position.signum() as u16;
            result += 1 - (sign * sign);
        }

        result
    }

    fn part_2(input: &Self::Input, _: &Self::Output1) -> Self::Output2 {
        let mut position = 50i16;
        let mut result = 0i16;

        for &rotation in input {
            result += ((position * rotation.signum() + 100) % 100 + rotation.abs()) / 100;
            position = (position + rotation).rem_euclid(100);
        }

        result as u16
    }
}
