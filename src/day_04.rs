#[cfg(test)]
fn get_positions(board: &Vec<Vec<String>>, pos: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut positions: Vec<Vec<(usize, usize)>> = Vec::new();
    if pos.0 + 3 < board.len() {
        positions.push(vec![
            (pos.0 + 1, pos.1),
            (pos.0 + 2, pos.1),
            (pos.0 + 3, pos.1),
        ]);
        if pos.1 + 3 < board[pos.0].len() {
            positions.push(vec![
                (pos.0 + 1, pos.1 + 1),
                (pos.0 + 2, pos.1 + 2),
                (pos.0 + 3, pos.1 + 3),
            ]);
        }
        if pos.1 >= 3 {
            positions.push(vec![
                (pos.0 + 1, pos.1 - 1),
                (pos.0 + 2, pos.1 - 2),
                (pos.0 + 3, pos.1 - 3),
            ]);
        }
    }
    if pos.0 >= 3 {
        positions.push(vec![
            (pos.0 - 1, pos.1),
            (pos.0 - 2, pos.1),
            (pos.0 - 3, pos.1),
        ]);
        if pos.1 + 3 < board[pos.0].len() {
            positions.push(vec![
                (pos.0 - 1, pos.1 + 1),
                (pos.0 - 2, pos.1 + 2),
                (pos.0 - 3, pos.1 + 3),
            ]);
        }
        if pos.1 >= 3 {
            positions.push(vec![
                (pos.0 - 1, pos.1 - 1),
                (pos.0 - 2, pos.1 - 2),
                (pos.0 - 3, pos.1 - 3),
            ]);
        }
    }
    if pos.1 + 3 < board[pos.0].len() {
        positions.push(vec![
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 + 2),
            (pos.0, pos.1 + 3),
        ]);
    }
    if pos.1 >= 3 {
        positions.push(vec![
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 - 2),
            (pos.0, pos.1 - 3),
        ]);
    }
    return positions;
}

#[cfg(test)]
fn xmas_count(board: &Vec<Vec<String>>, pos: (usize, usize)) -> u64 {
    let mut count = 0;
    let positions: Vec<Vec<(usize, usize)>> = get_positions(board, pos);
    for position_list in positions {
        let mut flag = true;
        for i in 0..3 {
            if "MAS"[i..i + 1] != board[position_list[i].0][position_list[i].1][0..1] {
                flag = false;
                break;
            }
        }
        if flag {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
fn is_x_origin(board: &Vec<Vec<String>>, pos: (usize, usize)) -> bool {
    return ((board[pos.0 - 1][pos.1 - 1] == "M" && board[pos.0 + 1][pos.1 + 1] == "S")
        || (board[pos.0 - 1][pos.1 - 1] == "S" && board[pos.0 + 1][pos.1 + 1] == "M"))
        && ((board[pos.0 - 1][pos.1 + 1] == "M" && board[pos.0 + 1][pos.1 - 1] == "S")
            || (board[pos.0 - 1][pos.1 + 1] == "S" && board[pos.0 + 1][pos.1 - 1] == "M"));
}

#[cfg(test)]
pub fn xmas_word_search(data: String) -> i64 {
    use crate::utils::read_lines_split_str;
    let board: Vec<Vec<String>> = read_lines_split_str(data, "")
        .into_iter()
        .map(|row| row.into_iter().filter(|col| col != "").collect())
        .collect();
    return (0..board.len())
        .into_iter()
        .map(|row| {
            (0..board[row].len())
                .into_iter()
                .filter_map(|col| match board[row][col].as_str() {
                    "X" => Some(col),
                    _ => None,
                })
                .map(|col| xmas_count(&board, (row, col)) as i64)
                .sum::<i64>()
        })
        .sum();
}

#[cfg(test)]
pub fn xmas_x_search(data: String) -> i64 {
    use crate::utils::read_lines_split_str;
    let board: Vec<Vec<String>> = read_lines_split_str(data, "")
        .into_iter()
        .map(|row| row.into_iter().filter(|col| col != "").collect())
        .collect();
    return (1..board.len() - 1)
        .into_iter()
        .map(|row| {
            (1..board[row].len() - 1)
                .into_iter()
                .filter_map(|col| match board[row][col].as_str() {
                    "A" => match is_x_origin(&board, (row, col)) {
                        true => Some(()),
                        false => None,
                    },
                    _ => None,
                })
                .count() as i64
        })
        .sum();
}
