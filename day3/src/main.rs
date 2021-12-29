fn main() {
    let input = include_str!("input.txt");
    let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon(input);
    let power_consumption = calculate_power_consumption(gamma_rate, epsilon_rate);
    println!("Part 1: {}, {}, {}", gamma_rate, epsilon_rate, power_consumption);
}

fn calculate_gamma_and_epsilon(input: &str) -> (u32, u32) {
    let bit_count = input.lines().next().unwrap().len();
    let num_records = input.lines().count();
    // When we do XOR later, only apply for maximum bits we have
    let max_bit_number = u32::pow(2, bit_count as _) - 1;

    let numbers: Vec<_> = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let gamma_rate = (0..bit_count).rev().map(|bit_position| {
        let column_filter = 1 << bit_position;
        // Bitwise OR to figure out how many ones in current column by counting any greater than
        // zero
        let entries_with_one = numbers.iter().filter(|n| (*n & column_filter) > 0).count();
        ((entries_with_one > num_records / 2) as u32, bit_position)
    }).fold(0, |acc, (most_common_bit, bit_position)| acc | (most_common_bit << bit_position));

    let epsilon_rate = gamma_rate ^ u32::MAX & max_bit_number;
    (gamma_rate, epsilon_rate)
}

fn calculate_power_consumption(gamma_rate: u32, epsilon_rate: u32) -> u32 {
    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon(input);
        let power_consumption = calculate_power_consumption(gamma_rate, epsilon_rate);
        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
        assert_eq!(power_consumption, 198);
    }
}
