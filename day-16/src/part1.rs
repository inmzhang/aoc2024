use glam::IVec2;
use pathfinding::prelude::dijkstra;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start: State = State::new(find_pos(&grid, 'S'), Direction::Right);
    let end: IVec2 = find_pos(&grid, 'E');
    let result =
        dijkstra(&start, |s| successor(s, &grid), |s| s.pos == end).expect("The path must exist");

    Ok(result.1.to_string())
}

fn successor(state: &State, grid: &[Vec<char>]) -> Vec<(State, usize)> {
    todo!()
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

fn in_bounds(pos: IVec2, grid: &[Vec<char>]) -> bool {
    pos.x >= 0 && pos.x < grid[0].len() as i32 && pos.y >= 0 && pos.y < grid.len() as i32
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
        "7036"
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
        "11048"
    )]

    fn test_process(#[case] input: &str, #[case] expect: &str) -> miette::Result<()> {
        assert_eq!(expect, process(input)?);
        Ok(())
    }
}
