#[cfg(test)]
pub fn clean_code(data: String) -> Vec<String> {
    use regex::Regex;
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(do|don\'t)\(\)").unwrap();
    re.find_iter(&data)
        .map(|m| m.as_str().to_string())
        .collect()
}

#[cfg(test)]
pub fn toggle_clean(expressions: Vec<String>) -> Vec<String> {
    let mut operations = Vec::new();
    let mut flag = true;
    for expression in expressions {
        if expression.starts_with("don't") {
            flag = false
        } else if expression.starts_with("do") {
            flag = true;
        } else if flag {
            operations.push(expression);
        }
    }
    operations
}

#[cfg(test)]
pub fn evaluate_expressions(input_expressions: Vec<String>, allow_toggle: bool) -> i64 {
    let expressions: Vec<String>;
    if allow_toggle {
        expressions = toggle_clean(input_expressions);
    } else {
        expressions = input_expressions
            .into_iter()
            .filter(|expr| expr.starts_with("mul"))
            .collect();
    }
    expressions
        .into_iter()
        .map(|s| (s[4..s.len() - 1]).to_string())
        .map(|nums| {
            nums.split(",")
                .map(|s| s.to_string())
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .map(|s_nums: Vec<i64>| s_nums[0] * s_nums[1])
        .sum()
}

#[cfg(test)]
pub fn evaluate_multiplication(data: String) -> i64 {
    evaluate_expressions(clean_code(data), false)
}

#[cfg(test)]
pub fn evaluate_multiplication_with_toggle(data: String) -> i64 {
    evaluate_expressions(clean_code(data), true)
}
