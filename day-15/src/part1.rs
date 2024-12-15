use std::collections::VecDeque;

use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut game) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    while !game.movements.is_empty() {
        game.step();
    }
    let res = game
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, element)| {
                if *element == Element::Box {
                    return Some(100 * y + x);
                }
                None
            })
        })
        .sum::<usize>();
    Ok(res.to_string())
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Element>>> {
    terminated(
        many1(terminated(
            many1(map(one_of("#.O@"), |c| match c {
                '#' => Element::Wall,
                '.' => Element::Empty,
                'O' => Element::Box,
                '@' => Element::Robot,
                _ => panic!("unexpected character: {}", c),
            })),
            line_ending,
        )),
        line_ending,
    )(input)
}

fn parse_move(input: &str) -> IResult<&str, VecDeque<Move>> {
    map(
        separated_list1(
            line_ending,
            many1(map(one_of("^v<>"), |c| match c {
                '^' => Move::Up,
                'v' => Move::Down,
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("unexpected character: {}", c),
            })),
        ),
        |nested| nested.into_iter().flatten().collect(),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Game> {
    let (input, grid) = parse_grid(input)?;
    let (input, movements) = parse_move(input)?;
    let robot = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, element)| {
                if *element == Element::Robot {
                    Some(IVec2::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();
    Ok((
        input,
        Game {
            grid,
            robot,
            movements,
        },
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Element {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn apply(&self, pos: IVec2) -> IVec2 {
        match self {
            Move::Up => pos - IVec2::new(0, 1),
            Move::Down => pos + IVec2::new(0, 1),
            Move::Left => pos - IVec2::new(1, 0),
            Move::Right => pos + IVec2::new(1, 0),
        }
    }
}

#[derive(Debug)]
struct Game {
    grid: Vec<Vec<Element>>,
    robot: IVec2,
    movements: VecDeque<Move>,
}

impl Game {
    fn at(&self, pos: IVec2) -> Element {
        self.grid[pos.y as usize][pos.x as usize]
    }

    fn set(&mut self, pos: IVec2, element: Element) {
        self.grid[pos.y as usize][pos.x as usize] = element;
    }

    fn step(&mut self) {
        let movement = self.movements.pop_front().expect("no more movements");
        let next_pos = movement.apply(self.robot);
        match self.at(next_pos) {
            Element::Empty => {
                self.set(next_pos, Element::Robot);
                self.set(self.robot, Element::Empty);
                self.robot = next_pos;
            }
            Element::Wall => {}
            Element::Box => {
                let mut pos = movement.apply(next_pos);
                let mut element = self.at(pos);
                while element != Element::Wall {
                    if element == Element::Empty {
                        self.set(pos, Element::Box);
                        self.set(next_pos, Element::Robot);
                        self.set(self.robot, Element::Empty);
                        self.robot = next_pos;
                        break;
                    }
                    pos = movement.apply(pos);
                    element = self.at(pos);
                }
            }
            _ => panic!("unexpected element"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!("2028", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input)?);
        Ok(())
    }
}
