use std::collections::HashSet;

use glam::IVec2;
use pathfinding::prelude::astar_bag_collect;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start: State = State::new(find_pos(&grid, 'S'), Direction::Right);
    let end: IVec2 = find_pos(&grid, 'E');
    let (paths, _) = astar_bag_collect(
        &start,
        |s| successor(s, &grid),
        |s| {
            let diff = (s.pos - end).abs();
            (diff.x + diff.y) as usize
        },
        |s| s.pos == end,
    )
    .expect("The path must exist");
    let res = paths
        .into_iter()
        .flat_map(|path| path.into_iter().map(|s| s.pos))
        .collect::<HashSet<IVec2>>();
    // print_path(&grid, &result.0);

    Ok(res.len().to_string())
}

#[allow(dead_code)]
fn print_path(grid: &[Vec<char>], path: &[State]) {
    let mut grid = grid.to_vec();
    for state in path {
        grid[state.pos.y as usize][state.pos.x as usize] = state.direction.get_char();
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn successor(state: &State, grid: &[Vec<char>]) -> Vec<(State, usize)> {
    state
        .direction
        .reachable()
        .into_iter()
        .filter_map(|d| {
            let new_pos = state.pos + d.get_ivec2();
            if !can_move(new_pos, grid) {
                return None;
            }
            let cost = if d == state.direction { 1 } else { 1001 };
            Some((State::new(new_pos, d), cost))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: IVec2,
    direction: Direction,
}

impl State {
    fn new(pos: IVec2, direction: Direction) -> Self {
        Self { pos, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reachable(&self) -> [Direction; 3] {
        match self {
            Self::Up => [Self::Left, Self::Up, Self::Right],
            Self::Down => [Self::Right, Self::Down, Self::Left],
            Self::Left => [Self::Down, Self::Left, Self::Up],
            Self::Right => [Self::Up, Self::Right, Self::Down],
        }
    }

    fn get_ivec2(&self) -> IVec2 {
        match self {
            Self::Up => IVec2::new(0, -1),
            Self::Down => IVec2::new(0, 1),
            Self::Left => IVec2::new(-1, 0),
            Self::Right => IVec2::new(1, 0),
        }
    }

    fn get_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

fn can_move(pos: IVec2, grid: &[Vec<char>]) -> bool {
    pos.x >= 0
        && pos.x < grid[0].len() as i32
        && pos.y >= 0
        && pos.y < grid.len() as i32
        && grid[pos.y as usize][pos.x as usize] != '#'
}

fn find_pos(grid: &[Vec<char>], c: char) -> IVec2 {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&ch| ch == c)
                .map(|x| IVec2::new(x as i32, y as i32))
        })
        .expect("should have the S or E")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        "45"
    )]
    #[case(
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        "64"
    )]

    fn test_process(#[case] input: &str, #[case] expect: &str) -> miette::Result<()> {
        assert_eq!(expect, process(input)?);
        Ok(())
    }
}
