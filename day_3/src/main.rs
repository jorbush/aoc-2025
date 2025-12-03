fn part_one(input_file_name: &str) -> u32 {
    let joltage_ratings = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let mut joltage = 0;
    for bank_joltage in joltage_ratings {
        let joltage_rating = bank_joltage
            .chars()
            .map(|line| line.to_string().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let first_max_val = *joltage_rating[..joltage_rating.len()-1].iter().max().unwrap();
        let max_index = joltage_rating.iter().position(|&x| x == first_max_val).unwrap();
        println!("{:?}", &joltage_rating[max_index+1..]);
        let second_max_val = *joltage_rating[max_index+1..]
            .iter()
            .max()
            .unwrap_or(&0);
        let jolts = format!(
            "{}{}",
            first_max_val,
            second_max_val
        ).parse::<u32>().unwrap();
        println!(
            "Bank joltage ratings: {:?}, max batteries combined rating: {}",
            bank_joltage, jolts
        );
        joltage += jolts;
    }
    joltage
}

fn main() {
    println!("----- AOC 2025 DAY 3 -----");
    println!("Part one answer: {}", part_one("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one("input_example.txt");
        assert_eq!(result, 357);
    }
}
