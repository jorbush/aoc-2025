fn part_one(input_file_name: &str) -> u64 {
    let mut fresh_ingredients_id_ranges = Vec::new();
    let mut available_ingredients: Vec<u64> = Vec::new();
    let mut got_all_ranges = false;
    for line in std::fs::read_to_string(input_file_name).unwrap().lines() {
        if line.trim().is_empty() {
            got_all_ranges = true;
            continue;
        }
        if !got_all_ranges {
            fresh_ingredients_id_ranges.push(line.to_string());
        } else {
            available_ingredients.push(line.trim().parse::<u64>().unwrap());
        }
    }
    println!(
        "Loaded {} ingredients ID ranges: {:?}",
        fresh_ingredients_id_ranges.len(),
        fresh_ingredients_id_ranges
    );
    println!(
        "Loaded {} available ingredients: {:?}",
        available_ingredients.len(),
        available_ingredients
    );
    let mut num_fresh_available_ingredients = 0;
    for available_ingredient in available_ingredients {
        for id_range in &fresh_ingredients_id_ranges {
            let mut parts = id_range.split("-");
            let min_id = parts.next().unwrap().parse::<u64>().unwrap();
            let max_id = parts.next().unwrap().parse::<u64>().unwrap();
            if available_ingredient >= min_id && available_ingredient <= max_id {
                num_fresh_available_ingredients += 1;
                println!(
                    "Ingredient {} is fresh (in range {}).",
                    available_ingredient, id_range
                );
                break;
            }
        }
    }
    num_fresh_available_ingredients
}

fn main() {
    println!("----- AOC 2025 DAY 6 -----");
    println!("Part one answer: {}", part_one("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one("input_example.txt");
        assert_eq!(result, 3);
    }
}
