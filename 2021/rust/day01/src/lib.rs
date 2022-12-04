//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test



pub fn process_part1(input: &str) -> String {
    input
    .lines()
    .collect::<Vec<&str>>()
    .windows(2)
    .map(|window| (window[0].trim().parse::<i32>().unwrap(), window[1].trim().parse::<i32>().unwrap()))
    .filter(|window| window.1 > window.0)
    .count()
    .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
    .lines()
    .collect::<Vec<&str>>()
    .windows(3)
    .map(|window| {
        // (window[0].trim().parse::<i32>().unwrap(), window[1].trim().parse::<i32>().unwrap())
        window.iter().map(|x| x.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>().iter().sum::<u32>()
    })
    .collect::<Vec<u32>>()
    .windows(2)
    .filter(|window| window[1] > window[0])
    .count()
    .to_string()
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
        assert_eq!(result, "5")
    }
}
