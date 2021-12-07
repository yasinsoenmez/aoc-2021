pub fn day1_1(input: &[u16]) -> usize {
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

pub fn day1_2(input: &[u16]) -> usize {
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
