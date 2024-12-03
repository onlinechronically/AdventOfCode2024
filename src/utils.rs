#[cfg(test)]
pub fn read_lines(data: &str) -> Vec<String> {
    return data.lines().map(|s| s.to_string()).collect();
}

#[cfg(test)]
pub fn read_lines_split(data: String, delimeter: &str) -> Vec<Vec<i64>> {
    return read_lines(&data)
        .into_iter()
        .map(|line| {
            line.split(delimeter)
                .map(|s| s.to_string().parse())
                .filter_map(|r| r.ok())
                .collect()
        })
        .collect();
}

#[cfg(test)]
pub fn read_lines_string(data: String) -> Vec<i64> {
    return read_lines(&data)
        .into_iter()
        .filter_map(|line| line.parse().ok())
        .collect();
}

#[cfg(test)]
pub fn read_lines_split_str(data: String, delimeter: &str) -> Vec<Vec<String>> {
    return read_lines(&data)
        .into_iter()
        .map(|line| line.split(delimeter).map(|s| s.to_string()).collect())
        .collect();
}
