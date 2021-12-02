#[derive(Clone, Copy)]
pub struct SubmarinePosition {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

pub fn day2_1(input: &str) -> u32 {
    let mut position = SubmarinePosition {horizontal: 0, depth: 0, aim: 0};

    let last_position =
        input
        .lines()
        .map(|line| {
            match line.split_once(' ') {
                Some(("forward", x)) => position.horizontal += x.parse::<u32>().unwrap(),
                Some(("down", x)) => position.depth += x.parse::<u32>().unwrap(),
                Some(("up", x)) => position.depth -= x.parse::<u32>().unwrap(),
                _ => (),
            }
            position
        })
        .last()
        .unwrap();
    last_position.horizontal * last_position.depth
}

pub fn day2_2(input: &str) -> u32 {
    let mut position = SubmarinePosition {horizontal: 0, depth: 0, aim: 0};

    let last_position =
        input
            .lines()
            .map(|line| {
                match line.split_once(' ') {
                    Some(("forward", x)) => {
                        let x_parsed = x.parse::<u32>().unwrap();
                        position.horizontal += x_parsed;
                        position.depth += position.aim * x_parsed;
                    },
                    Some(("down", x)) => position.aim += x.parse::<u32>().unwrap(),
                    Some(("up", x)) => position.aim -= x.parse::<u32>().unwrap(),
                    _ => (),
                }
                position
            })
            .last()
            .unwrap();
    last_position.horizontal * last_position.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        let day2 = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        assert_eq!(day2_1(&day2), 150);
        assert_eq!(day2_2(&day2), 900);
    }
}