fn part_one(input_file_name: &str) -> u32 {
    let rotations = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let dial: Vec<u32> = (0..100).collect();
    let dial_len = dial.len() as u32;
    let mut current_position = 50;
    println!("The dial starts by pointing at {}.", current_position);
    let mut zero_matches = 0;
    for rotation in rotations {
        let (direction, distance_str) = rotation.split_at(1);
        let distance = distance_str.parse::<u32>().unwrap();
        match direction {
            "L" => {
                current_position = (current_position + dial_len - (distance % dial_len)) % dial_len;
            }
            "R" => {
                current_position = (current_position + distance) % dial_len;
            }
            _ => panic!("Invalid direction"),
        }
        if current_position == 0 {
            zero_matches += 1;
        }
        println!(
            "The dial is rotated {} to point at {}.",
            rotation, current_position
        );
    }
    zero_matches
}

fn part_two(input_file_name: &str) -> u32 {
    let rotations = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let dial: Vec<u32> = (0..100).collect();
    let dial_len = dial.len() as u32;
    let mut current_position = 50;
    println!("The dial starts by pointing at {}.", current_position);
    let mut zero_matches = 0;
    for rotation in rotations {
        let (direction, distance_str) = rotation.split_at(1);
        let distance = distance_str.parse::<u32>().unwrap();
        let pass_zero_times;
        match direction {
            "L" => {
                let inverted_pos = (dial_len - current_position) % dial_len;
                pass_zero_times = (inverted_pos + distance) / dial_len;
                current_position = (current_position + dial_len - (distance % dial_len)) % dial_len;
            }
            "R" => {
                pass_zero_times = (current_position + distance) / dial_len;
                current_position = (current_position + distance) % dial_len;
            }
            _ => panic!("Invalid direction"),
        }
        zero_matches += pass_zero_times;
        // println!("pass_zero_times: {}", pass_zero_times);
        if pass_zero_times > 0 {
            let times_message = if pass_zero_times == 1 {
                "once"
            } else {
                &format!("{} times", pass_zero_times)
            };
            println!(
                "The dial is rotated {} to point at {}; during this rotation, it points at 0 {}.",
                rotation, current_position, times_message
            );
        } else {
            println!(
                "The dial is rotated {} to point at {}.",
                rotation, current_position
            );
        }
    }
    zero_matches
}

fn main() {
    println!("----- AOC 2025 DAY 1 -----");
    println!("Part one answer: {}", part_one("input.txt"));
    println!("Part two answer: {}", part_two("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one("test_input.txt"), 3);
    }

    #[test]
    fn test_part_one_real_input() {
        assert_eq!(part_one("input.txt"), 1150);
    }

    #[test]
    fn test_part_one_tricky_input_step() {
        assert_eq!(part_one("input_tricky.txt"), 0);
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(part_two("test_input.txt"), 6);
    }

    #[test]
    fn test_part_two_basic_input() {
        assert_eq!(part_two("input_basic_2.txt"), 10);
    }

    #[test]
    fn test_part_two_real_input() {
        assert_eq!(part_two("input.txt"), 6738);
    }
}
