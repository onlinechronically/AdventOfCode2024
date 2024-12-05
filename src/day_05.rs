#[cfg(test)]
fn is_correct_order(queue: &[i64], rules: Option<&Vec<i64>>) -> bool {
    match rules {
        Some(lor) => lor.iter().all(|n| !queue.contains(n)),
        None => true,
    }
}

#[cfg(test)]
fn correct_prints(s: String) -> Vec<Vec<i64>> {
    use crate::utils::{filter_lines, read_lines_split};
    use std::{collections::HashMap, vec};
    let original = read_lines_split(filter_lines(s.clone(), |line| line.contains(",")), ",");
    let order = read_lines_split(filter_lines(s.clone(), |line| line.contains("|")), "|");
    let mut order_map: HashMap<i64, Vec<i64>> = HashMap::new();
    order
        .iter()
        .for_each(|pair| match order_map.get_mut(&pair[0]) {
            Some(lon) => lon.push(pair[1]),
            None => {
                order_map.insert(pair[0], vec![pair[1].clone()]);
            }
        });
    return original
        .into_iter()
        .filter(|queue| {
            (0..queue.len())
                .into_iter()
                .all(|i| is_correct_order(&queue[0..i + 1], order_map.get(&queue[i])))
        })
        .collect();
}

#[cfg(test)]
fn incorrect_prints(s: String) -> Vec<Vec<i64>> {
    use crate::utils::{filter_lines, read_lines_split};
    use std::{collections::HashMap, vec};
    let original = read_lines_split(filter_lines(s.clone(), |line| line.contains(",")), ",");
    let order = read_lines_split(filter_lines(s.clone(), |line| line.contains("|")), "|");
    let mut order_map: HashMap<i64, Vec<i64>> = HashMap::new();
    order
        .iter()
        .for_each(|pair| match order_map.get_mut(&pair[0]) {
            Some(lon) => lon.push(pair[1]),
            None => {
                order_map.insert(pair[0], vec![pair[1].clone()]);
            }
        });
    return original
        .into_iter()
        .filter(|queue| {
            (0..queue.len())
                .into_iter()
                .any(|i| !is_correct_order(&queue[0..i + 1], order_map.get(&queue[i])))
        })
        .collect();
}

#[cfg(test)]
fn correct_print_order(
    queue: Vec<i64>,
    order_map: &std::collections::HashMap<i64, Vec<i64>>,
) -> Vec<i64> {
    let mut new_queue: Vec<i64> = Vec::new();

    queue.iter().for_each(|n| match new_queue.is_empty() {
        true => new_queue.push(*n),
        false => match order_map.get(&n) {
            Some(dependencies) => {
                let mut flag = false;
                for i in 0..new_queue.len() {
                    if dependencies.contains(&new_queue[i]) {
                        new_queue.insert(i, *n);
                        flag = true;
                        break;
                    }
                }
                match flag {
                    false => new_queue.push(*n),
                    _ => {}
                }
            }
            None => new_queue.push(*n),
        },
    });

    return new_queue;
}

#[cfg(test)]
fn print_order_correction(s: String) -> Vec<Vec<i64>> {
    use crate::utils::{filter_lines, read_lines_split};
    use std::collections::HashMap;
    let original = incorrect_prints(s.clone());
    let order = read_lines_split(filter_lines(s, |line| line.contains("|")), "|");
    let mut order_map: HashMap<i64, Vec<i64>> = HashMap::new();
    order
        .iter()
        .for_each(|pair| match order_map.get_mut(&pair[0]) {
            Some(lon) => lon.push(pair[1]),
            None => {
                order_map.insert(pair[0], vec![pair[1].clone()]);
            }
        });
    return original
        .iter()
        .map(|print_queue| correct_print_order(print_queue.to_vec(), &order_map))
        .collect();
}

#[cfg(test)]
pub fn valid_middle_job_sum(s: String) -> i64 {
    return correct_prints(s)
        .into_iter()
        .map(|lon| lon[lon.len() / 2])
        .sum();
}

#[cfg(test)]
pub fn correct_prints_sum(s: String) -> i64 {
    return print_order_correction(s)
        .into_iter()
        .map(|lon| lon[lon.len() / 2])
        .sum();
}
