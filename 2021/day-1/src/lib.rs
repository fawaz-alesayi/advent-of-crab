use std::fs;

// Split the input into an array of strings by line
fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_first_line() {
        // read the file "day-1.txt" line by line
        let lines = read_lines("day-1.txt");
        assert_eq!(lines[0], "182");
    }

    #[test]
    fn it_counts_increases() {
        // read the file "day-1.txt" line by line
        let lines = read_lines("day-1.txt").into_iter();
        let lines_without_first = read_lines("day-1.txt").into_iter().skip(1);

        let mut count = 0;

        // get two lines at a time and compare which is the largest
        lines.zip(lines_without_first).for_each(|(a, b)| {
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            if b > a {
                count += 1;
            };
        });

        assert_eq!(count, 1215);
    }

    #[test]
    fn it_reads_three_lines() {
        let lines = read_lines("day-1.txt").into_iter();
        let lines_without_first = read_lines("day-1.txt").into_iter().skip(1);
        let lines_without_first_two = read_lines("day-1.txt").into_iter().skip(2);

        let mut tuplet = lines
            .zip(lines_without_first)
            .zip(lines_without_first_two)
            .map(|((x, y), z)| (x, y, z));

        assert_eq!(
            tuplet.next().unwrap(),
            (hstr("182"), hstr("188"), hstr("204"))
        );
    }

    #[test]
    fn it_counts_increases_in_a_three_measurement_sliding_window() {
        let lines = read_lines("day-1.txt").into_iter();
        let lines_without_first = read_lines("day-1.txt").into_iter().skip(1);
        let lines_without_first_two = read_lines("day-1.txt").into_iter().skip(2);

        let mut prev: Option<i32> = None;
        let mut count = 0;
        lines
            .zip(lines_without_first)
            .zip(lines_without_first_two)
            .map(|((x, y), z)| {
                x.parse::<i32>().unwrap() + y.parse::<i32>().unwrap() + z.parse::<i32>().unwrap()
            })
            .for_each(|sum| {
                if let Some(p) = prev {
                    if sum > p {
                        count += 1;
                    }
                }
                prev = Some(sum);
            });

        assert_eq!(count, 1150);
    }
}

fn hstr(s: &str) -> String {
    s.to_string()
}
