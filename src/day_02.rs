use crate::day::Day;

pub struct Day02;

impl Day for Day02 {
    type Input = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn id() -> String {
        "02".to_string()
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .split(",")
            .map(|raw| raw.split("-").map(|n| n.parse::<u64>().unwrap()))
            .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
            .collect()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut result = 0u64;

        for &(from, to) in input {
            let from_ord = from.ilog10() + 1;
            let to_ord = to.ilog10() + 1;

            for ord in from_ord / 2..=to_ord / 2 {
                let mag = 10u64.pow(ord);

                for prefix in (from / mag).max(mag / 10)..mag {
                    let n = prefix * mag + prefix;
                    if n > to {
                        break;
                    }
                    if n >= from {
                        result += n;
                    }
                }
            }
        }

        result
    }

    fn part_2(input: &Self::Input, _: &Self::Output1) -> Self::Output2 {
        let mut result = 0u64;

        for &(from, to) in input {
            for n in from..=to {
                let n_ord = n.ilog10() + 1;

                for ord in 1..=n_ord / 2 {
                    if n_ord % ord != 0 {
                        continue;
                    }

                    let mask = 10u64.pow(ord);
                    let expected = n % mask;

                    let mut rem = n / mask;
                    let mut is_bad_id = false;

                    while rem > 0 {
                        if rem % mask != expected {
                            is_bad_id = false;
                            break;
                        }

                        is_bad_id = true;
                        rem /= mask;
                    }

                    if is_bad_id {
                        result += n;
                        break;
                    }
                }
            }
        }

        result
    }
}
