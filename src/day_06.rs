use crate::day::Day;

pub struct Day06;

impl Day for Day06 {
    type Input = String;
    type Output1 = u64;
    type Output2 = u64;

    fn id() -> String {
        "06".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input.to_owned()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut lines = input.lines();

        let operands: Vec<Vec<_>> = lines
            .by_ref()
            .take(4)
            .map(|l| {
                l.split(" ")
                    .filter(|l| !l.is_empty())
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect()
            })
            .collect();

        let operations: Vec<_> = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|l| !l.is_empty())
            .collect();

        let mut result = 0u64;
        for i in 0..operations.len() {
            result += match operations[i] {
                "+" => operands[0][i] + operands[1][i] + operands[2][i] + operands[3][i],
                _ => operands[0][i] * operands[1][i] * operands[2][i] * operands[3][i],
            }
        }

        result
    }

    fn part_2(input: &Self::Input, _: Self::Output1) -> Self::Output2 {
        let mut lines = input.lines();

        let operand_lines: Vec<_> = lines.by_ref().take(4).collect();
        let operations_line = lines.next().unwrap();

        let mut i = 0;
        let mut result = 0u64;

        while i < operations_line.len() {
            let operation = &operations_line[i..i + 1];

            let j = i;
            i += 1;
            while i < operations_line.len() && &operations_line[i..i + 1] == " " {
                i += 1;
            }

            if i == operations_line.len() {
                i += 2;
            }

            let mut l_res = if operation == "+" { 0 } else { 1 };
            for q in j..i - 1 {
                let mut operand_str = String::new();
                for d in 0..4 {
                    operand_str += &operand_lines[d][q..q + 1];
                }

                let operand: u64 = operand_str.trim().parse().unwrap();

                if operation == "+" {
                    l_res += operand;
                } else {
                    l_res *= operand;
                }
            }

            result += l_res;
        }

        result
    }
}
