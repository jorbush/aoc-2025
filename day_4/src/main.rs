fn part_one(input_file_name: &str) -> u32 {
    let grid_papers = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let mut num_accessible_papers = 0;
    for (grid_index, line) in grid_papers.iter().enumerate() {
        // println!("Loaded grid paper: {}", line);
        for (ch_index, ch) in line.chars().enumerate() {
            if ch == '@' {
                // it is accessible if it has less than four '@' neighbors (up, down, left, right, diagonals)
                let mut adjancent = 0;
                //left
                if ch_index != 0 {
                    if line.chars().nth(ch_index - 1).unwrap() == '@' {
                        adjancent += 1;
                    }
                }
                // right
                // check if not last character
                if ch_index != line.len() - 1 {
                    if line.chars().nth(ch_index + 1).unwrap() == '@' {
                        adjancent += 1;
                    }
                }
                // check top line
                if grid_index != 0 {
                    let top_line = &grid_papers[grid_index - 1];
                    // top
                    if top_line.chars().nth(ch_index).unwrap() == '@' {
                        adjancent += 1;
                    }
                    // top-left
                    if ch_index != 0 {
                        if top_line.chars().nth(ch_index - 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                    // top-right
                    if ch_index != top_line.len() - 1 {
                        if top_line.chars().nth(ch_index + 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                }
                // bottom line
                // check if not last line
                if grid_index != grid_papers.len() - 1 {
                    let bottom_line = &grid_papers[grid_index + 1];
                    // bottom
                    if bottom_line.chars().nth(ch_index).unwrap() == '@' {
                        adjancent += 1;
                    }
                    // bottom-left
                    if ch_index != 0 {
                        if bottom_line.chars().nth(ch_index - 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                    // bottom-right
                    if ch_index != bottom_line.len() - 1 {
                        if bottom_line.chars().nth(ch_index + 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                }
                if adjancent < 4 {
                    num_accessible_papers += 1;
                }
            }
        }
    }
    num_accessible_papers
}

fn remove_accessible_papers(grid_papers: &mut Vec<String>) -> u32 {
    let mut num_accessible_papers = 0;
    let mut positions_to_remove = Vec::new();

    for (grid_index, line) in grid_papers.iter().enumerate() {
        // println!("Loaded grid paper: {}", line);
        for (ch_index, ch) in line.chars().enumerate() {
            if ch == '@' {
                // it is accessible if it has less than four '@' neighbors (up, down, left, right, diagonals)
                let mut adjancent = 0;
                //left
                if ch_index != 0 {
                    if line.chars().nth(ch_index - 1).unwrap() == '@' {
                        adjancent += 1;
                    }
                }
                // right
                // check if not last character
                if ch_index != line.len() - 1 {
                    if line.chars().nth(ch_index + 1).unwrap() == '@' {
                        adjancent += 1;
                    }
                }
                // check top line
                if grid_index != 0 {
                    let top_line = &grid_papers[grid_index - 1];
                    // top
                    if top_line.chars().nth(ch_index).unwrap() == '@' {
                        adjancent += 1;
                    }
                    // top-left
                    if ch_index != 0 {
                        if top_line.chars().nth(ch_index - 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                    // top-right
                    if ch_index != top_line.len() - 1 {
                        if top_line.chars().nth(ch_index + 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                }
                // bottom line
                // check if not last line
                if grid_index != grid_papers.len() - 1 {
                    let bottom_line = &grid_papers[grid_index + 1];
                    // bottom
                    if bottom_line.chars().nth(ch_index).unwrap() == '@' {
                        adjancent += 1;
                    }
                    // bottom-left
                    if ch_index != 0 {
                        if bottom_line.chars().nth(ch_index - 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                    // bottom-right
                    if ch_index != bottom_line.len() - 1 {
                        if bottom_line.chars().nth(ch_index + 1).unwrap() == '@' {
                            adjancent += 1;
                        }
                    }
                }
                if adjancent < 4 {
                    num_accessible_papers += 1;
                    positions_to_remove.push((grid_index, ch_index));
                }
            }
        }
    }

    // remove accessible papers
    for (grid_index, ch_index) in positions_to_remove {
        let line = &grid_papers[grid_index];
        let mut new_line = line.clone();
        new_line.replace_range(ch_index..ch_index + 1, ".");
        grid_papers[grid_index] = new_line;
    }

    num_accessible_papers
}

fn part_two(input_file_name: &str) -> u32 {
    let mut grid_papers = std::fs::read_to_string(input_file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect::<Vec<String>>();
    let mut num_accessible_papers = 0;
    loop {
        let current_accessible = remove_accessible_papers(&mut grid_papers);
        if current_accessible == 0 {
            break;
        }
        num_accessible_papers += current_accessible;
    }
    num_accessible_papers
}

fn main() {
    println!("----- AOC 2025 DAY 4 -----");
    println!("Part one answer: {}", part_one("input.txt"));
    println!("Part two answer: {}", part_two("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let result = part_one("input_example.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_one() {
        let result = part_one("input.txt");
        assert_eq!(result, 1389);
    }

    #[test]
    fn test_part_two_example() {
        let result = part_two("input_example.txt");
        assert_eq!(result, 43);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("input.txt");
        assert_eq!(result, 9000);
    }
}
