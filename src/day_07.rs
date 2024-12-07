#[cfg(test)]
struct LineEntry {
    target: i64,
    numbers: Vec<i64>,
    includes_concat: bool,
}

#[cfg(test)]
fn all_operators<'a>(n: usize, includes_concat: bool) -> Vec<Vec<&'a str>> {
    if n == 0 {
        return vec![];
    } else if n == 1 {
        match includes_concat {
            true => vec![vec!["+"], vec!["*"], vec!["||"]],
            false => vec![vec!["+"], vec!["*"]],
        }
    } else {
        let r = all_operators(n - 1, includes_concat);
        let l: Vec<Vec<&str>> = r
            .into_iter()
            .flat_map(|mut operators| {
                let mut other = operators.clone();
                let mut concat_other = operators.clone();
                operators.push("+");
                other.push("*");
                concat_other.push("||");
                match includes_concat {
                    true => vec![operators, other, concat_other],
                    false => vec![operators, other],
                }
            })
            .collect();
        return l;
    }
}

#[cfg(test)]
fn apply_operators(numbers: &Vec<i64>, operators: &Vec<&str>) -> i64 {
    if numbers.len() == 1 {
        return numbers[0];
    } else {
        let mut n = numbers[0];
        for i in 1..numbers.len() {
            let op = operators[i - 1];
            let num = numbers[i];
            match op {
                "+" => n += num,
                "*" => n *= num,
                _ => n = (n.to_string() + &num.to_string()).parse().unwrap(),
            }
        }
        return n;
    }
}

#[cfg(test)]
impl LineEntry {
    fn is_valid(&self) -> bool {
        return all_operators(self.numbers.len() - 1, self.includes_concat)
            .into_iter()
            .map(|operators| apply_operators(&self.numbers, &operators))
            .any(|total_sum| total_sum == self.target);
    }
}

#[cfg(test)]
pub fn sum_valid_equations(s: String, includes_concat: bool) -> i64 {
    use crate::utils::read_lines_split_str;
    return read_lines_split_str(s, ": ")
        .into_iter()
        .map(|line| LineEntry {
            target: line[0].parse().unwrap(),
            numbers: line[1].split(" ").map(|n| n.parse().unwrap()).collect(),
            includes_concat,
        })
        .filter(|line| line.is_valid())
        .map(|line| line.target)
        .sum();
}
