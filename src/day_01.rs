use std::collections::HashMap;

#[allow(dead_code)]
pub fn sum_differences(lists: String) -> Result<i64, ()> {
    let mut difference_sum: i64 = 0;
    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    for line in lists.lines() {
        let mut line_arr = line.split("   ");
        let first_str_maybe = line_arr.next();
        let second_str_maybe = line_arr.next();
        let first_num: i64;
        let second_num: i64;
        match first_str_maybe {
            Some(s) => {
                let num_maybe = s.parse();
                match num_maybe {
                    Ok(n) => first_num = n,
                    Err(_) => return Err(()),
                }
            }
            None => return Err(()),
        }
        match second_str_maybe {
            Some(s) => {
                let num_maybe = s.parse();
                match num_maybe {
                    Ok(n) => second_num = n,
                    Err(_) => return Err(()),
                }
            }
            None => return Err(()),
        }
        list_one.push(first_num);
        list_two.push(second_num);
    }
    list_one.sort();
    list_two.sort();
    for i in 0..list_one.len() {
        let curr_first: &i64;
        let curr_second: &i64;
        if let Some(n) = list_one.get(i) {
            curr_first = n;
        } else {
            return Err(());
        }
        if let Some(n) = list_two.get(i) {
            curr_second = n;
        } else {
            return Err(());
        }
        let difference = curr_first - curr_second;
        difference_sum += difference.abs();
    }
    Ok(difference_sum)
}

#[allow(dead_code)]
pub fn similarity_sum(lists: String) -> Result<i64, ()> {
    let mut similarity_map: HashMap<&i64, i64> = HashMap::new();
    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    for line in lists.lines() {
        let mut line_arr = line.split("   ");
        let first_str_maybe = line_arr.next();
        let second_str_maybe = line_arr.next();
        let first_num: i64;
        let second_num: i64;
        match first_str_maybe {
            Some(s) => {
                let num_maybe = s.parse();
                match num_maybe {
                    Ok(n) => first_num = n,
                    Err(_) => return Err(()),
                }
            }
            None => return Err(()),
        }
        match second_str_maybe {
            Some(s) => {
                let num_maybe = s.parse();
                match num_maybe {
                    Ok(n) => second_num = n,
                    Err(_) => return Err(()),
                }
            }
            None => return Err(()),
        }
        list_one.push(first_num);
        list_two.push(second_num);
    }
    for i in 0..list_two.len() {
        let curr_first: &i64;
        if let Some(n) = list_one.get(i) {
            curr_first = n;
        } else {
            return Err(());
        }
        match similarity_map.get(curr_first) {
            Some(n) => similarity_map.insert(curr_first, n + 1),
            None => similarity_map.insert(curr_first, 1),
        };
    }
    Ok(list_two
        .iter()
        .map(|n| (n, similarity_map.get(n)))
        .filter(|(_, v)| v.is_some())
        .map(|(k, v)| k * v.unwrap())
        .sum())
}
