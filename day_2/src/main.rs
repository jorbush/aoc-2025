fn part_one(input_file_name: &str) -> u64 {
    let id_ranges = std::fs::read_to_string(input_file_name)
        .unwrap()
        .replace("\n", "")
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("Loaded {} ID ranges: {:?}", id_ranges.len(), id_ranges);
    let mut sum_invalid_ids: u64 = 0;
    for id_range in id_ranges {
        let parts: Vec<&str> = id_range.split("-").collect();
        let (first_id, last_id) = (
            parts[0].parse::<u64>().unwrap(),
            parts[1].parse::<u64>().unwrap(),
        );
        let mut current_invalid_ids: Vec<String> = Vec::new();
        let mut current_sum = 0;
        for current_id in first_id..=last_id {
            let current_id_str = current_id.to_string();
            if current_id_str.len() % 2 != 0 {
                continue; // Skip IDs with odd length
            }
            let (first_part, last_part) = current_id_str.split_at(current_id_str.len() / 2);
            //println!(
            //  "Checking ID {}: first part {}, last part {}",
            //   current_id, first_part, last_part
            //);
            if first_part == last_part {
                current_invalid_ids.push(current_id_str);
                current_sum += current_id;
            }
        }
        if current_sum == 0 {
            println!("{} contains no invalid IDs.", id_range)
        } else {
            println!(
                "{} has {} invalid IDs: {:?}.",
                id_range,
                current_invalid_ids.len(),
                current_invalid_ids
            );
        }
        sum_invalid_ids += current_sum;
    }
    sum_invalid_ids
}

fn part_two(input_file_name: &str) -> u64 {
    let id_ranges = std::fs::read_to_string(input_file_name)
        .unwrap()
        .replace("\n", "")
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("Loaded {} ID ranges: {:?}", id_ranges.len(), id_ranges);
    let mut sum_invalid_ids: u64 = 0;
    for id_range in id_ranges {
        let parts: Vec<&str> = id_range.split("-").collect();
        let (first_id, last_id) = (
            parts[0].parse::<u64>().unwrap(),
            parts[1].parse::<u64>().unwrap(),
        );
        let mut current_invalid_ids: Vec<String> = Vec::new();
        let mut current_sum = 0;
        for current_id in first_id..=last_id {
            let current_id_str = current_id.to_string();
            // TODO: an invalid id is one made only of some sequence of digits repeated at least twice
            let (first_part, last_part) = current_id_str.split_at(current_id_str.len() / 2);
            println!(
                "Checking ID {}: first part {}, last part {}",
                current_id, first_part, last_part
            );
            if first_part == last_part {
                current_invalid_ids.push(current_id_str);
                current_sum += current_id;
            }
        }
        if current_sum == 0 {
            println!("{} contains no invalid IDs.", id_range)
        } else {
            println!(
                "{} has {} invalid IDs: {:?}.",
                id_range,
                current_invalid_ids.len(),
                current_invalid_ids
            );
        }
        sum_invalid_ids += current_sum;
    }
    sum_invalid_ids
}

fn main() {
    println!("----- AOC 2025 DAY 2 -----");
    println!("Part one answer: {}", part_one("input.txt"));
    println!("Part two answer: {}", part_two("example_input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one("example_input.txt");
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_one() {
        let result = part_one("input.txt");
        assert_eq!(result, 54234399924);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two("example_input.txt");
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("input.txt");
        assert_eq!(result, 0);
    }
}
