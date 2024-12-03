#[cfg(test)]
pub fn sum_differences(lists: String) -> i64 {
    use crate::utils::read_lines_split;

    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    let lines = read_lines_split(lists, "   ");
    for pair in lines {
        list_one.push(pair[0]);
        list_two.push(pair[1]);
    }
    list_one.sort();
    list_two.sort();
    return (0..list_one.len())
        .map(|i| list_one[i] - list_two[i])
        .map(i64::abs)
        .sum();
}

#[cfg(test)]
pub fn similarity_sum(lists: String) -> i64 {
    use crate::utils::read_lines_split;
    use std::collections::HashMap;

    let mut similarity_map: HashMap<i64, i64> = HashMap::new();
    let mut list_one: Vec<i64> = Vec::new();
    let mut list_two: Vec<i64> = Vec::new();
    let lines = read_lines_split(lists, "   ");
    for pair in lines {
        list_one.push(pair[0]);
        list_two.push(pair[1]);
    }
    list_one.into_iter().for_each(|n| {
        match similarity_map.get(&n) {
            Some(n) => similarity_map.insert(*n, n + 1),
            None => similarity_map.insert(n, 1),
        };
    });
    list_two
        .iter()
        .map(|n| (n, similarity_map.get(n)))
        .filter(|(_, v)| v.is_some())
        .map(|(k, v)| k * v.unwrap())
        .sum()
}
