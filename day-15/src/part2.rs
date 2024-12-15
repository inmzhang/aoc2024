use std::collections::{HashSet, VecDeque};

use glam::IVec2;
use itertools::repeat_n;
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
    // for i in 0..10 {
    //     println!("Step: {}: {:?}", i, game.movements[0]);
    //     game.step();
    //     println!("------------------");
    //     print_grid(&game.grid);
    // }
    while !game.movements.is_empty() {
        game.step();
    }
    // print_grid(&game.grid);
    let res = game
        .grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, element)| {
                if *element == Element::BoxLeft {
                    return Some(100 * y + x);
                }
                None
            })
        })
        .sum::<usize>();
    Ok(res.to_string())
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Element>]) {
    for row in grid {
        for element in row {
            print!(
                "{}",
                match element {
                    Element::Empty => '.',
                    Element::Wall => '#',
                    Element::BoxLeft => '[',
                    Element::BoxRight => ']',
                    Element::Robot => '@',
                }
            );
        }
        println!();
    }
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<Element>>> {
    terminated(
        many1(terminated(
            many1(map(one_of("#.O@"), move |c| match c {
                '#' => Element::Wall,
                '.' => Element::Empty,
                'O' => Element::BoxLeft,
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

fn expand_grid(grid: Vec<Vec<Element>>) -> Vec<Vec<Element>> {
    grid.into_iter()
        .map(|row| {
            let mut expanded_row = Vec::with_capacity(row.len() * 2);
            row.into_iter().for_each(|e| match e {
                Element::BoxLeft => expanded_row.extend([Element::BoxLeft, Element::BoxRight]),
                Element::Empty => expanded_row.extend(repeat_n(Element::Empty, 2)),
                Element::Wall => expanded_row.extend(repeat_n(Element::Wall, 2)),
                Element::Robot => expanded_row.extend([Element::Robot, Element::Empty]),
                _ => panic!("unexpected element"),
            });
            expanded_row
        })
        .collect()
}

fn parse(input: &str) -> IResult<&str, Game> {
    let (input, grid) = parse_grid(input)?;
    let grid = expand_grid(grid);
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
    BoxLeft,
    BoxRight,
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

#[derive(Debug, Clone)]
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

    fn can_move_box(&self, left_pos: IVec2, movement: Move) -> Option<HashSet<IVec2>> {
        let next_left = movement.apply(left_pos);
        let next_right = movement.apply(left_pos + IVec2::new(1, 0));
        match movement {
            Move::Up | Move::Down => match (self.at(next_left), self.at(next_right)) {
                (Element::Empty, Element::Empty) => Some(HashSet::from_iter([left_pos])),
                (Element::Wall, _) | (_, Element::Wall) => None,
                (Element::BoxLeft, _) => self.can_move_box(next_left, movement).map(|mut set| {
                    set.insert(left_pos);
                    set
                }),
                (Element::BoxRight, e) => {
                    let mut set = HashSet::new();
                    set.insert(left_pos);
                    if let Some(left_set) =
                        self.can_move_box(next_left + IVec2::new(-1, 0), movement)
                    {
                        set.extend(left_set);
                    } else {
                        return None;
                    }
                    if e == Element::BoxLeft {
                        if let Some(right_set) = self.can_move_box(next_right, movement) {
                            set.extend(right_set);
                        } else {
                            return None;
                        }
                    }
                    Some(set)
                }
                (_, Element::BoxLeft) => self.can_move_box(next_right, movement).map(|mut set| {
                    set.insert(left_pos);
                    set
                }),
                _ => panic!("unexpected element"),
            },
            Move::Left => match self.at(next_left) {
                Element::Empty => Some(HashSet::from_iter([left_pos])),
                Element::Wall => None,
                Element::BoxRight => self
                    .can_move_box(left_pos + IVec2::new(-2, 0), movement)
                    .map(|mut set| {
                        set.insert(left_pos);
                        set
                    }),
                _ => panic!("unexpected element"),
            },
            Move::Right => match self.at(next_right) {
                Element::Empty => Some(HashSet::from_iter([left_pos])),
                Element::Wall => None,
                Element::BoxLeft => self
                    .can_move_box(left_pos + IVec2::new(2, 0), movement)
                    .map(|mut set| {
                        set.insert(left_pos);
                        set
                    }),
                _ => panic!("unexpected element {:?}", self.at(next_right)),
            },
        }
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
            Element::BoxLeft | Element::BoxRight => {
                let left_pos = if self.at(next_pos) == Element::BoxLeft {
                    next_pos
                } else {
                    next_pos - IVec2::new(1, 0)
                };
                if let Some(set) = self.can_move_box(left_pos, movement) {
                    let mut new_grid = self.clone();
                    set.iter().for_each(|p| {
                        new_grid.set(*p, Element::Empty);
                        new_grid.set(*p + IVec2::new(1, 0), Element::Empty);
                    });
                    set.into_iter().for_each(|p| {
                        new_grid.set(movement.apply(p), Element::BoxLeft);
                        new_grid.set(movement.apply(p) + IVec2::new(1, 0), Element::BoxRight);
                    });
                    self.grid = new_grid.grid;
                    self.set(next_pos, Element::Robot);
                    self.set(self.robot, Element::Empty);
                    self.robot = next_pos;
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
    fn test_process() -> miette::Result<()> {
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
        assert_eq!("9021", process(input)?);
        Ok(())
    }
}
