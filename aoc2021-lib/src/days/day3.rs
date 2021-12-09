pub fn day3_parse(input: &str) -> (Vec<u16>, usize) {
    let (first, _) = input.split_once('\n').unwrap();
    let bit_length = first.trim().len();

    (
        input
            .lines()
            .map(|line| u16::from_str_radix(line, 2).unwrap())
            .collect(),
        bit_length,
    )
}

fn calculate_gamma_epsilon(input: &(Vec<u16>, usize)) -> (u16, u16) {
    let (input, bit_length) = input;
    let input_middle = (input.len() / 2) as u16;
    let mut bit_count = vec![0_u16; *bit_length];
    let mut gamma = 0;
    let mut epsilon = 0;

    for num in input {
        for i in 0..*bit_length {
            if num & (1 << i) != 0 {
                bit_count[i] += 1;
            }
        }
    }

    for (i, count) in bit_count.into_iter().enumerate() {
        if count > input_middle {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
}

pub fn day3_1(input: &(Vec<u16>, usize)) -> u32 {
    let (gamma_rate, epsilon_rate) = calculate_gamma_epsilon(input);

    gamma_rate as u32 * epsilon_rate as u32
}

fn get_bit_list(input: Vec<u16>,  bit_place: usize, bit_length: usize) -> (Vec<u16>, Vec<u16>) {
    let mut bit_one_list = Vec::with_capacity(input.len());
    let mut bit_zero_list = Vec::with_capacity(input.len());

    for num in input {
        if num & (1 << bit_length - bit_place - 1) != 0 {
            bit_one_list.push(num);
        } else {
            bit_zero_list.push(num);
        }
    }
    (bit_one_list, bit_zero_list)
}

fn determine_oxygen_rating(oxygen_list: Vec<u16>,  bit_place: usize, bit_length: usize) -> Vec<u16> {
    let (bit_one_list, bit_zero_list) = get_bit_list(oxygen_list, bit_place, bit_length);

    if bit_one_list.len() >= bit_zero_list.len() {
        bit_one_list
    } else {
        bit_zero_list
    }
}

fn determine_scrubber_rating(scrubber_list: Vec<u16>, bit_place: usize, bit_length: usize) -> Vec<u16> {
    let (bit_one_list, bit_zero_list) = get_bit_list(scrubber_list, bit_place, bit_length);

    if bit_one_list.len() < bit_zero_list.len() {
        bit_one_list
    } else {
        bit_zero_list
    }
}

fn calculate_oxygen_scrubber(input: &(Vec<u16>, usize)) -> (u16, u16) {
    let (input, bit_length) = input;

    let mut oxygen_list = input.clone();
    let mut scrubber_list = input.clone();

    for i in 0..*bit_length {
        if oxygen_list.len() > 1 {
            oxygen_list = determine_oxygen_rating(oxygen_list, i, *bit_length);
        }
        if scrubber_list.len() > 1 {
            scrubber_list = determine_scrubber_rating(scrubber_list, i, *bit_length);
        }
    }
    (*oxygen_list.first().unwrap(), *scrubber_list.first().unwrap())
}

pub fn day3_2(input: &(Vec<u16>, usize)) -> u32 {
    let (oxygen_rate, scrubber_rate) = calculate_oxygen_scrubber(input);

    oxygen_rate as u32 * scrubber_rate as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let day3 = day3_parse(input);
        assert_eq!(day3_1(&day3), 198);
    }

    #[test]
    fn d3p2() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let day3 = day3_parse(input);
        assert_eq!(day3_2(&day3), 230);
    }
}
