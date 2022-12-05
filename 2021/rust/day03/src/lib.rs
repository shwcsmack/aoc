//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test
mod matrix;

use matrix::Matrix;

pub fn process_part1(input: &str) -> String {
    let matrix = input.parse::<Matrix>().unwrap();
    matrix.power_consumption().to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
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

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "198");
    }

    #[ignore]
    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "0")
    }
}
