pub enum Movement {
    Forward(u8),
    Down(u8),
    Up(u8),
}

fn to_movement(movement: &str) -> Movement {
    match movement.split_once(' ') {
        Some(("forward", x)) => Movement::Forward(x.parse().unwrap()),
        Some(("down", x)) => Movement::Down(x.parse().unwrap()),
        Some(("up", x)) => Movement::Up(x.parse().unwrap()),
        _ => Movement::Forward(0),
    }
}

pub fn day2_parse(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(to_movement)
        .collect()
}

pub fn day2_1(input: &[Movement]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for movement in input.iter() {
        match movement {
            Movement::Forward(x) => horizontal += *x as u32,
            Movement::Down(x) => depth += *x as u32,
            Movement::Up(x) => depth -= *x as u32,
        }
    }
    horizontal * depth
}

pub fn day2_2(input: &[Movement]) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for movement in input.iter() {
        match movement {
            Movement::Forward(x) => {
                horizontal += *x as u32;
                depth += aim * *x as u32;
            }
            Movement::Down(x) => aim += *x as u32,
            Movement::Up(x) => aim -= *x as u32,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        let day2 = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let day2_parsed = day2_parse(day2);

        assert_eq!(day2_1(&day2_parsed), 150);
    }

    #[test]
    fn d2p2() {
        let day2 = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let day2_parsed = day2_parse(day2);

        assert_eq!(day2_2(&day2_parsed), 900);
    }
}
