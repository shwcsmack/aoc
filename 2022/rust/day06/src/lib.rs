//cargo run --bin part-1
//cargo run --bin part-2
//cargo watch -x check -x test

fn has_duplicates(chars_in: &[char]) -> bool {
    let mut chars_to_check = chars_in.clone().to_vec();
    let mut output = false;

    while chars_to_check.len() > 0 && !output {
        let char_to_check = chars_to_check.pop().unwrap();
        output = chars_to_check.contains(&char_to_check);
    }
    output
}

pub fn process_part1(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();

    let pre_marker_windows: Vec<&[char]> = chars.windows(4).take_while(|window| has_duplicates(window)).collect();

    (pre_marker_windows.len() + 4).to_string()
}

pub fn process_part2(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();

    let pre_marker_windows: Vec<&[char]> = chars.windows(14).take_while(|window| has_duplicates(window)).collect();

    (pre_marker_windows.len() + 14).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part_one() {
        let result = process_part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn part_two() {
        let result = process_part2(INPUT);
        assert_eq!(result, "19")
    }
}
