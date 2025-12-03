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
        let original_length = joltage_rating.len();
        let mut max_batteries = vec![0u32; original_length];
        let mut joltage_rating = joltage_rating;
        for _ in 0..2 {
            let max_val = *joltage_rating.iter().max().unwrap();
            let max_index = joltage_rating.iter().position(|&x| x == max_val).unwrap();
            let max_rating = joltage_rating.remove(max_index);
            max_batteries[max_index] = max_rating;
        }
        let mut jolts_str = String::new();
        println!("Max batteries selected: {:?}", max_batteries);
        for jolt in max_batteries.iter() {
            if jolt != &(0 as u32) {
                jolts_str.push_str(&jolt.to_string());
            }
        }
        let jolts = jolts_str.parse::<u32>().unwrap();
        println!(
            "Bank joltage ratings: {:?}, max batteries combined rating: {}",
            bank_joltage, jolts
        );
        joltage += jolts;
    }
    joltage
}

fn main() {
    println!("----- AOC 2025 DAY 2 -----");
    println!("Part one answer: {}", part_one("input_example.txt"));
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
