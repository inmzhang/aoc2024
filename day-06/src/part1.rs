use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut grid = vec![];
    let mut i = 0;
    let mut j = 0;
    for (row, line) in input.lines().enumerate() {
        let mut v = vec![];
        for (col, c) in line.chars().enumerate() {
            if c == '^' {
                i = row as i32;
                j = col as i32;
                v.push(0);
            } else {
                v.push(if c == '#' { 1 } else { 0 });
            }
        }
        grid.push(v);
    }
    let width = grid[0].len();
    let height = grid.len();

    let mut track: HashSet<(i32, i32)> = HashSet::new();
    track.insert((i, j));
    let mut direction = Direction::Up;
    loop {
        let next = direction.moving(i, j);
        if out_of_bounds(next.0, next.1, width, height) {
            break;
        }
        if grid[next.0 as usize][next.1 as usize] == 1 {
            direction = direction.turn_right();
            continue;
        }
        track.insert(next);
        (i, j) = next;
    }

    Ok(track.len().to_string())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn moving(&self, i: i32, j: i32) -> (i32, i32) {
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn out_of_bounds(i: i32, j: i32, width: usize, height: usize) -> bool {
    i < 0 || j < 0 || i >= height as i32 || j >= width as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
