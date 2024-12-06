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
    let cur = Rec {
        i,
        j,
        direction: Direction::Up,
    };
    // first traverse to get the track
    let (track, is_trapped) = traverse(&grid, cur);
    let track_pos: HashSet<(i32, i32)> = track.iter().map(|rec| rec.pos()).collect();
    debug_assert!(!is_trapped);
    let mut count = 0;
    for pos in track_pos {
        if pos == cur.pos() {
            continue;
        }
        grid[pos.0 as usize][pos.1 as usize] = 1;
        let (_, is_trapped) = traverse(&grid, cur);
        if is_trapped {
            count += 1;
        }
        grid[pos.0 as usize][pos.1 as usize] = 0;
    }

    Ok(count.to_string())
}

fn traverse(grid: &[Vec<i32>], cur: Rec) -> (HashSet<Rec>, bool) {
    let width = grid[0].len();
    let height = grid.len();
    let mut cur = cur;

    let mut track: HashSet<Rec> = HashSet::new();
    track.insert(cur);
    loop {
        let next = cur.direction.moving(&cur);
        if out_of_bounds(&next, width, height) {
            break (track, false);
        }
        if grid[next.i as usize][next.j as usize] == 1 {
            cur = Rec {
                i: cur.i,
                j: cur.j,
                direction: cur.direction.turn_right(),
            };
            continue;
        }
        if track.contains(&next) {
            break (HashSet::new(), true);
        }
        track.insert(next);
        cur = next;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rec {
    i: i32,
    j: i32,
    direction: Direction,
}

impl Rec {
    fn pos(&self) -> (i32, i32) {
        (self.i, self.j)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn moving(&self, rec: &Rec) -> Rec {
        match self {
            Direction::Up => Rec {
                i: rec.i - 1,
                j: rec.j,
                direction: rec.direction,
            },
            Direction::Down => Rec {
                i: rec.i + 1,
                j: rec.j,
                direction: rec.direction,
            },
            Direction::Left => Rec {
                i: rec.i,
                j: rec.j - 1,
                direction: rec.direction,
            },
            Direction::Right => Rec {
                i: rec.i,
                j: rec.j + 1,
                direction: rec.direction,
            },
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

fn out_of_bounds(record: &Rec, width: usize, height: usize) -> bool {
    record.i < 0 || record.j < 0 || record.i >= height as i32 || record.j >= width as i32
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
