fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);
    println!("{}", times_depth_increases(&input));

    println!("{}", times_depth_increases_sliding_window(&input));
}

fn parse_input(input: &str) -> Vec<i64> {
   input.lines()
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

fn times_depth_increases(input: &[i64]) -> usize {
    input
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn times_depth_increases_sliding_window(input: &[i64]) -> usize {
    let a = input
        .windows(3)
        .map::<i64, _>(|window| window.iter().sum())
        .collect::<Vec<_>>();

    times_depth_increases(&a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(times_depth_increases(&parse_input(input)), 7);
    }

    #[test]
    fn example_input_window() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(times_depth_increases_sliding_window(&parse_input(input)), 5);
    }
}
