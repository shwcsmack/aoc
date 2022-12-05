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

    const INPUT: &str = "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "0")
    }
}
