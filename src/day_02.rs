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

#[allow(dead_code)]
pub fn total_safe_reports(reports: String, is_strict: bool) -> Result<i64, ()> {
    let mut total = 0;
    let mut lines = reports.lines();
    loop {
        match lines.next() {
            Some(line) => {
                let mut str_nums = line.split(" ");
                let mut nums: Vec<i64> = Vec::new();
                loop {
                    match str_nums.next() {
                        Some(curr) => match curr.parse() {
                            Ok(n) => nums.push(n),
                            Err(_) => return Err(()),
                        },
                        None => break,
                    }
                }
                let mut modifications: Vec<Vec<i64>> = Vec::new();
                for i in 0..nums.len() {
                    let mut new_list = nums.clone();
                    new_list.remove(i);
                    modifications.push(new_list);
                }
                if is_safe(nums) {
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
            None => break,
        }
    }
    return Ok(total);
}
