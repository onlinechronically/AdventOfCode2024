#[cfg(test)]
pub fn is_safe(nums: Vec<i64>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let is_increasing = nums.get(0).unwrap() <= nums.get(1).unwrap();
    let mut flag = true;
    for i in 1..nums.len() {
        if is_increasing {
            let diff = nums.get(i).unwrap() - nums.get(i - 1).unwrap();
            if nums.get(i) < nums.get(i - 1) || diff.abs() < 1 || diff.abs() > 3 {
                flag = false;
                break;
            }
        } else {
            let diff = nums.get(i - 1).unwrap() - nums.get(i).unwrap();
            if nums.get(i) > nums.get(i - 1) || diff.abs() < 1 || diff.abs() > 3 {
                flag = false;
                break;
            }
        }
    }
    return flag;
}

#[cfg(test)]
pub fn total_safe_reports(reports: String, is_strict: bool) -> i64 {
    use crate::utils::read_lines_split;

    let mut total = 0;
    let lines = read_lines_split(reports, " ");
    for line_data in lines {
        let modifications: Vec<Vec<i64>> = (0..line_data.len())
            .map(|index| {
                line_data
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, v)| v.to_owned())
                    .collect()
            })
            .collect();
        if is_safe(line_data) {
            total += 1;
        } else if !is_strict
            && modifications
                .iter()
                .map(|lon| is_safe(lon.to_vec()))
                .any(|b| b)
        {
            total += 1;
        }
    }
    return total;
}
