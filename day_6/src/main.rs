fn part_one(input_file_name: &str) -> u64 {
    let mut input_lines = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let operations_line = input_lines.pop().unwrap();
    let operations = operations_line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let numbers = input_lines.clone();
    let mut total_sum = 0;
    for operation_index in 0..operations.len() {
        let operation = operations.get(operation_index).unwrap();
        let mut operation_str = String::new();
        let mut operation_result = if operation.to_string() == "+" { 0 } else { 1 };
        for numbers_index in 0..numbers.len() {
            let numbers_line = numbers.get(numbers_index).unwrap();
            let numbers_split = numbers_line.split_whitespace().collect::<Vec<&str>>();
            let number_str = numbers_split.get(operation_index).unwrap();
            let number = number_str.parse::<u64>().unwrap();
            if operation.to_string() == "+" {
                operation_result += number;
            } else {
                operation_result *= number;
            }
            if numbers_index == numbers.len() - 1 {
                operation_str += &format!("{}", number_str);
            } else {
                operation_str += &format!("{} {} ", number_str, operation);
            }
        }
        println!("{} = {}", operation_str, operation_result);
        total_sum += operation_result;
    }
    total_sum
}

fn part_two(input_file_name: &str) -> u64 {
    let mut input_lines = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let operations_line = input_lines.pop().unwrap();
    let operations = operations_line
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>();
    let numbers = input_lines.clone();
    let mut total_sum = 0;

    for operation_index in 0..operations.len() {
        let operation = operations.get(operation_index).unwrap();
        let mut operation_str = String::new();
        let mut operation_result = if operation.to_string() == "+" { 0 } else { 1 };
        let mut operations_numbers = Vec::new();
        for numbers_index in 0..numbers.len() {
            let numbers_line = numbers.get(numbers_index).unwrap();
            let chars: Vec<char> = numbers_line.chars().collect();
            let chunk_size = if operations.len() > 0 {
                (chars.len() + operations.len() - 1) / operations.len()
            } else {
                chars.len()
            };
            let chunk_chars = chars.chunks(chunk_size).nth(operation_index).unwrap_or(&[]);
            let number_str: String = chunk_chars.iter().collect();
            operations_numbers.push(number_str.to_string());
        }
        println!("Operations numbers: {:?}", operations_numbers);
        let max_number_length = operations_numbers
            .iter()
            .map(|num_str| num_str.len())
            .max()
            .unwrap();
        let mut real_numbers = Vec::new();
        for ind_number in 0..max_number_length {
            let mut number_str = String::new();
            for num in operations_numbers.iter() {
                if let Some(ch) = num.chars().nth(ind_number) {
                    if !ch.is_whitespace() {
                        number_str.push(ch);
                    }
                }
            }
            if !number_str.is_empty() {
                real_numbers.push(number_str.parse::<u64>().unwrap());
            }
        }
        for (num_ind, real_num) in real_numbers.iter().enumerate() {
            if operation.to_string() == "+" {
                operation_result += real_num;
            } else {
                operation_result *= real_num;
            }
            if num_ind == numbers.len() - 1 {
                operation_str += &format!("{}", real_num);
            } else {
                operation_str += &format!("{} {} ", real_num, operation);
            }
        }
        println!("{} = {}", operation_str, operation_result);
        total_sum += operation_result;
    }
    total_sum
}

fn main() {
    println!("----- AOC 2025 DAY 6 -----");
    println!("Part one answer: {}", part_one("input.txt"));
    println!("Part two answer: {}", part_two("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one("input_example.txt");
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_one() {
        let result = part_one("input.txt");
        assert_eq!(result, 4583860641327);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two("input_example.txt");
        assert_eq!(result, 3263827);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("input.txt");
        assert_eq!(result, 4583860641327);
    }
}
