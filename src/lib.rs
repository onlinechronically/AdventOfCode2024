mod day_01;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use day_01::{similarity_sum, sum_differences};

    #[test]
    fn test_day_01() {
        let input_read = fs::read_to_string("files/day_01.txt");
        let output_read = fs::read_to_string("files/day_01_ans.txt");
        let mut output_one: i64 = -1;
        let mut output_two: i64 = -1;
        match output_read {
            Ok(output_txt) => {
                let mut lines = output_txt.lines();
                match lines.next() {
                    Some(line) => match line.parse() {
                        Ok(n) => {
                            output_one = n;
                        }
                        Err(_) => (),
                    },
                    None => (),
                }
                match lines.next() {
                    Some(line) => match line.parse() {
                        Ok(n) => {
                            output_two = n;
                        }
                        Err(_) => (),
                    },
                    None => (),
                }
            }
            Err(_) => (),
        }
        if let Ok(input_txt) = input_read {
            let mut output = sum_differences(input_txt.clone());
            match output {
                Ok(n) => assert_eq!(output_one, n),
                Err(_) => assert!(false),
            }
            output = similarity_sum(input_txt.clone());
            match output {
                Ok(n) => assert_eq!(output_two, n),
                Err(_) => assert!(false),
            }
        } else {
            panic!("There was anerror reading the input for this puzzle");
        }
    }
}
