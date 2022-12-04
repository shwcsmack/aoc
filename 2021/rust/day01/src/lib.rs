//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test



pub fn process_part1(input: &str) -> String {
    todo!();
}

pub fn process_part2(input: &str) -> String {
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "0")
    }
}
