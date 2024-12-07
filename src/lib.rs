mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod utils;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use day_01::{similarity_sum, sum_differences};
    use day_02::total_safe_reports;
    use day_03::{evaluate_multiplication, evaluate_multiplication_with_toggle};
    use day_04::{xmas_word_search, xmas_x_search};
    use day_05::{correct_prints_sum, valid_middle_job_sum};
    use day_06::{get_tiles_touched, MazeResult, MazeType};
    use day_07::sum_valid_equations;
    use utils::read_lines_string;

    const DAY: u8 = 7;
    const DEBUG: bool = false;

    #[test]
    fn test_day() {
        let input_read: String;
        if DEBUG {
            input_read = fs::read_to_string(format!("files/temp.txt")).unwrap();
        } else {
            input_read = fs::read_to_string(format!("files/day_{:0>2}.txt", DAY)).unwrap();
        }
        let output_read = read_lines_string(
            fs::read_to_string(format!("files/day_{:0>2}_ans.txt", DAY)).unwrap(),
        );
        let output_one: i64 = if !DEBUG { output_read[0] } else { -1 };
        let output_two: i64 = if !DEBUG { output_read[1] } else { -1 };
        if DAY == 1 {
            assert_eq!(output_one, sum_differences(input_read.clone()));
            assert_eq!(output_two, similarity_sum(input_read));
        } else if DAY == 2 {
            assert_eq!(output_one, total_safe_reports(input_read.clone(), true));
            assert_eq!(output_two, total_safe_reports(input_read, false));
        } else if DAY == 3 {
            assert_eq!(output_one, evaluate_multiplication(input_read.clone()));
            assert_eq!(output_two, evaluate_multiplication_with_toggle(input_read));
        } else if DAY == 4 {
            assert_eq!(output_one, xmas_word_search(input_read.clone()));
            assert_eq!(output_two, xmas_x_search(input_read));
        } else if DAY == 5 {
            assert_eq!(output_one, valid_middle_job_sum(input_read.clone()));
            assert_eq!(output_two, correct_prints_sum(input_read));
        } else if DAY == 6 {
            assert_eq!(
                MazeResult::Tiles(output_one),
                get_tiles_touched(MazeType::String(input_read.clone()), false)
            );
        } else if DAY == 7 {
            assert_eq!(output_one, sum_valid_equations(input_read.clone(), false));
            assert_eq!(output_two, sum_valid_equations(input_read.clone(), true));
        } else {
            dbg!();
        }
    }
}
