use std::str::FromStr;

pub fn day1_parse(input: &str) -> Vec<u16> {
    input
        .lines()
        .flat_map(u16::from_str)
        .collect()
}

fn count_depth_increase(input: &[u16], step: usize) -> usize {
    input
        .windows(step)
        .filter(|x| x.last() > x.first())
        .count()
}

pub fn day1_1(input: &[u16]) -> usize {
    count_depth_increase(input, 2)
}

pub fn day1_2(input: &[u16]) -> usize {
    count_depth_increase(input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        let day1 = day1_parse(input);
        assert_eq!(day1_1(&day1), 7);
    }

    #[test]
    fn d1p2() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        let day1 = day1_parse(input);
        assert_eq!(day1_2(&day1), 5);
    }
}
