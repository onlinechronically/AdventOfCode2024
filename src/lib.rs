mod day_01;
mod day_02;
mod utils;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use day_01::{similarity_sum, sum_differences};
    use day_02::total_safe_reports;
    use utils::read_lines_string;

    const DAY: u8 = 3;

    #[test]
    fn test_day() {
        let input_read = fs::read_to_string(format!("files/day_{:0>2}.txt", DAY)).unwrap();
        let output_read = read_lines_string(
            fs::read_to_string(format!("files/day_{:0>2}_ans.txt", DAY)).unwrap(),
        );
        let output_one: i64 = output_read[0];
        let output_two: i64 = output_read[1];
        if DAY == 1 {
            assert_eq!(output_one, sum_differences(input_read.clone()));
            assert_eq!(output_two, similarity_sum(input_read));
        } else if DAY == 2 {
            assert_eq!(output_one, total_safe_reports(input_read.clone(), true));
            assert_eq!(output_two, total_safe_reports(input_read, false));
        } else {
            dbg!();
        }
    }
}
