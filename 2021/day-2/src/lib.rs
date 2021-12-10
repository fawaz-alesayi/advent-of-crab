#[cfg(test)]
mod tests {
    use std::fs;

    // Split the input into an array of strings by line
    fn read_lines(filename: &str) -> Vec<String> {
        fs::read_to_string(filename)
            .expect("Something went wrong reading the file")
            .lines()
            .map(|l| l.to_string())
            .collect()
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Operation {
        Forward(i32),
        Down(i32),
        Up(i32),
    }

    fn parse_lines(lines: &Vec<String>) -> Vec<Operation> {
        lines
            .iter()
            .map(|l| {
                let words = l.split_whitespace().collect::<Vec<&str>>();
                let direction = words[0];
                let units = words[1].parse::<i32>().unwrap();

                match direction {
                    "forward" => Operation::Forward(units),
                    "up" => Operation::Up(units),
                    "down" => Operation::Down(units),
                    _ => panic!("Unknown direction {}", direction),
                }
            })
            .collect()
    }

    fn calculate_horizontal(operations: &Vec<Operation>) -> i32 {
        let mut x = 0;
        for operation in operations {
            match operation {
                Operation::Forward(units) => x += units,
                _ => {}
            }
        }
        x
    }

    fn calculate_depth(operations: &Vec<Operation>) -> i32 {
        let mut y = 0;
        for operation in operations {
            match operation {
                Operation::Up(units) => y -= units,
                Operation::Down(units) => y += units,
                _ => {}
            }
        }
        y
    }

    fn calculate_depth_using_aim(operations: &Vec<Operation>) -> i32 {
        let mut aim = 0;
        let mut depth = 0;
        for operation in operations {
            match operation {
                Operation::Up(units) => aim -= units,
                Operation::Down(units) => aim += units,
                Operation::Forward(units) => {
                    depth += aim * units
                }
            }
        }
        depth
    }

    #[test]
    fn it_reads_the_first_line() {
        let lines = read_lines("input.txt");
        assert_eq!(lines[0], "forward 9");
    }

    #[test]
    fn it_parses_the_first_line() {
        let lines = vec![String::from("forward 9")];
        let operations = parse_lines(&lines);
        assert_eq!(operations[0], Operation::Forward(9));
    }

    #[test]
    fn it_determines_the_final_horizontal_position() {
        let lines = read_lines("input.txt");
        let operations = parse_lines(&lines);
        let horizontal_position = calculate_horizontal(&operations);
        assert_eq!(horizontal_position, 2024);
    }

    #[test]
    fn it_determines_the_depth() {
        let lines = read_lines("input.txt");
        let operations = parse_lines(&lines);
        let vertical_position = calculate_depth(&operations);
        assert_eq!(vertical_position, 717);
    }

    #[test]
    fn depth_by_horizontal_position() {
        let lines = read_lines("input.txt");
        let operations = parse_lines(&lines);
        let horizontal_position = calculate_horizontal(&operations);
        let vertical_position = calculate_depth(&operations);
        assert_eq!(horizontal_position * vertical_position, 1451208);
    }

    #[test]
    fn it_determines_depth_using_aim() {
        let lines = read_lines("input.txt");
        let operations = parse_lines(&lines);
        let depth = calculate_depth_using_aim(&operations);
        assert_eq!(depth, 800465);
    }

    #[test]
    fn depth_using_aim_by_horizontal_position() {
        let lines = read_lines("input.txt");
        let operations = parse_lines(&lines);
        let horizontal_position = calculate_horizontal(&operations);
        let depth = calculate_depth_using_aim(&operations);
        assert_eq!(horizontal_position * depth, 1620141160);
    }
}
