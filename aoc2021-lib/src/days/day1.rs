use std::str::FromStr;

pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(i32::from_str)
        .collect()
}

pub fn day1_1(input: &[i32]) -> usize {
    let mut prev = input[0];

    input
        .iter()
        .filter(|x| {
            let has_depth_increased = x > &&prev;
            prev = **x;

            has_depth_increased
        })
        .count()
}

pub fn day1_2(input: &[i32]) -> usize {
    let mut tripple_sum = input[0] + input[1] + input[2];

    input
        .windows(3)
        .filter(|x| {
            let new_sum = x.iter().sum();
            let has_depth_increased = new_sum > tripple_sum;
            tripple_sum = new_sum;

            has_depth_increased
        })
        .count()
}

pub fn day1_1_alt(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|x| {
            x.last() > x.first()
        })
        .count()
}

pub fn day1_2_alt(input: &[i32]) -> usize {
    input
        .windows(4)
        .filter(|x| {
            x.last() > x.first()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        let day1 = parse(input);
        assert_eq!(day1_1(&day1), 7);
    }

    #[test]
    fn d1p2() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        let day1 = parse(input);
        assert_eq!(day1_2(&day1), 5);
    }
}
