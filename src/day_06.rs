#[cfg(test)]
fn find_guard(grid: &Vec<Vec<String>>) -> (usize, usize) {
    let guard_symbols = ["V", "<", ">", "^"];
    (0..grid.len())
        .into_iter()
        .map(|row| {
            (0..grid[row].len())
                .into_iter()
                .filter(move |col| guard_symbols.contains(&grid[row][*col].as_str()))
                .map(move |col| (col, row))
        })
        .flatten()
        .next()
        .unwrap()
}

#[cfg(test)]
fn get_guard_symbol(grid: &Vec<Vec<String>>) -> Option<&str> {
    let guard_symbols = ["V", "<", ">", "^"];
    (0..grid.len())
        .into_iter()
        .map(|row| {
            (0..grid[row].len())
                .into_iter()
                .filter(move |col| guard_symbols.contains(&grid[row][*col].as_str()))
                .map(move |col| grid[row][col].as_str())
        })
        .flatten()
        .next()
}

#[cfg(test)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
enum Move {
    Forward(Direction),
    Rotation(),
}

#[cfg(test)]
fn get_next_move(grid: &mut Vec<Vec<String>>, guard_pos: (usize, usize)) -> Move {
    match get_guard_symbol(grid).unwrap() {
        "V" => {
            if guard_pos.1 < grid[guard_pos.1].len() - 1 {
                if grid[guard_pos.1 + 1][guard_pos.0] == "#" {
                    return Move::Rotation();
                } else {
                    return Move::Forward(Direction::Down);
                }
            } else {
                return Move::Forward(Direction::Down);
            }
        }
        "<" => {
            if guard_pos.1 > 0 {
                if grid[guard_pos.1][guard_pos.0 - 1] == "#" {
                    return Move::Rotation();
                } else {
                    return Move::Forward(Direction::Left);
                }
            } else {
                return Move::Forward(Direction::Left);
            }
        }
        ">" => {
            if guard_pos.0 < grid.len() - 1 {
                if grid[guard_pos.1][guard_pos.0 + 1] == "#" {
                    return Move::Rotation();
                } else {
                    return Move::Forward(Direction::Right);
                }
            } else {
                return Move::Forward(Direction::Right);
            }
        }
        _ => {
            if guard_pos.1 > 0 {
                if grid[guard_pos.1 - 1][guard_pos.0] == "#" {
                    return Move::Rotation();
                } else {
                    return Move::Forward(Direction::Up);
                }
            } else {
                return Move::Forward(Direction::Up);
            }
        }
    }
    // look
}

#[cfg(test)]
fn next_rotation(s: &str) -> String {
    match s {
        "V" => "<".to_string(),
        "<" => "^".to_string(),
        "^" => ">".to_string(),
        _ => "V".to_string(),
    }
}

#[cfg(test)]
#[derive(Debug, PartialEq)]
pub enum MazeResult {
    Tiles(i64),
    Cycle(bool),
}

#[cfg(test)]
#[derive(Debug)]
pub enum MazeType<'a> {
    String(String),
    Vec(&'a mut Vec<Vec<String>>),
}

#[cfg(test)]
pub fn get_tiles_touched(s: MazeType, cycle_detection: bool) -> MazeResult {
    use std::i64;

    use crate::utils::read_lines_split_str;
    let mut grid: Vec<Vec<String>>;

    match s {
        MazeType::String(maze_string) => {
            grid = read_lines_split_str(maze_string, "")
                .into_iter()
                .map(|line| line.into_iter().filter(|s| s != "").collect())
                .collect()
        }
        MazeType::Vec(maze_arr) => grid = maze_arr.to_vec(),
    }

    let mut guard_pos = {
        let (pos_x, pos_y) = find_guard(&grid);
        (pos_x as i64, pos_y as i64)
    };
    let mut iter: u64 = 0;
    while guard_pos.1 >= 0
        && guard_pos.0 >= 0
        && guard_pos.1 < grid.len() as i64
        && guard_pos.0 < grid[guard_pos.1 as usize].len() as i64
        && get_guard_symbol(&grid).is_some()
        && (!cycle_detection || iter < 10000)
    {
        let next_move = get_next_move(&mut grid, (guard_pos.0 as usize, guard_pos.1 as usize));
        match next_move {
            Move::Rotation() => {
                grid[guard_pos.1 as usize][guard_pos.0 as usize] =
                    next_rotation(grid[guard_pos.1 as usize][guard_pos.0 as usize].as_str())
            }
            Move::Forward(direction) => match direction {
                Direction::Up => {
                    if guard_pos.1 - 1 >= 0 {
                        grid[guard_pos.1 as usize - 1][guard_pos.0 as usize] =
                            grid[guard_pos.1 as usize][guard_pos.0 as usize].clone();
                    }
                    grid[guard_pos.1 as usize][guard_pos.0 as usize] = "X".to_string();
                    guard_pos.1 -= 1;
                }
                Direction::Down => {
                    if guard_pos.1 as usize + 1 < grid.len() - 1 {
                        grid[(guard_pos.1 as usize) + 1][guard_pos.0 as usize] =
                            grid[guard_pos.1 as usize][guard_pos.0 as usize].clone();
                    }
                    grid[guard_pos.1 as usize][guard_pos.0 as usize] = "X".to_string();
                    guard_pos.1 += 1;
                }
                Direction::Left => {
                    if guard_pos.0 - 1 >= 0 {
                        grid[guard_pos.1 as usize][(guard_pos.0 as usize) - 1] =
                            grid[guard_pos.1 as usize][guard_pos.0 as usize].clone();
                    }
                    grid[guard_pos.1 as usize][guard_pos.0 as usize] = "X".to_string();
                    guard_pos.0 -= 1;
                }
                Direction::Right => {
                    if guard_pos.0 as usize + 1 < grid[guard_pos.1 as usize].len() - 1 {
                        grid[guard_pos.1 as usize][guard_pos.0 as usize + 1] =
                            grid[guard_pos.1 as usize][guard_pos.0 as usize].clone();
                    }
                    grid[guard_pos.1 as usize][guard_pos.0 as usize] = "X".to_string();
                    guard_pos.0 += 1;
                }
            },
        }
        iter += 1;
    }
    if cycle_detection {
        return MazeResult::Cycle(iter == 10000);
    } else {
        return MazeResult::Tiles(
            grid.iter()
                .map(|row| row.iter().filter(|s| *s == "X").count() as i64)
                .sum::<i64>()
                + 1,
        );
    }
}

#[cfg(test)]
pub fn get_all_empty_pos(s: String) -> Vec<(usize, usize)> {
    use crate::utils::read_lines_split_str;
    let grid: Vec<Vec<String>> = read_lines_split_str(s, "")
        .into_iter()
        .map(|line| line.into_iter().filter(|s| s != "").collect())
        .collect();

    return (0..grid.len())
        .flat_map(|row| {
            (0..grid[row].len())
                .filter(|col| grid[row][*col] == ".")
                .map(|col| (row, col))
                .collect::<Vec<_>>()
        })
        .collect();
}

#[cfg(test)]
pub fn part_two(s: String) -> i64 {
    use crate::utils::read_lines_split_str;
    let spots = get_all_empty_pos(s.clone());
    let mut traps = 0;
    for empty_pos in spots {
        let mut grid: Vec<Vec<String>> = read_lines_split_str(s.clone(), "")
            .into_iter()
            .map(|line| line.into_iter().filter(|s| s != "").collect())
            .collect();
        grid[empty_pos.1][empty_pos.0] = String::from("#");
        let has_cycle = get_tiles_touched(MazeType::Vec(&mut grid), true);
        match has_cycle {
            MazeResult::Cycle(cycle) => match cycle {
                true => {
                    traps += 1;
                }
                _ => {}
            },
            _ => {}
        }
    }
    // create grids and put a trap at every possible spot
    return traps as i64;
}
