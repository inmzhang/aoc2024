use std::collections::HashMap;

use glam::IVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut robots) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let max_iter = 101 * 103;
    let mut min_safety_factor = usize::MAX;
    let mut min_safety_factor_step = 0;
    let mut min_safety_factor_grid = robots.clone();
    for i in 1..=max_iter {
        robots.iter_mut().for_each(|robot| robot.step());
        let mut count: HashMap<u8, usize> = HashMap::new();
        robots
            .iter()
            .filter_map(|robot| robot.quadrant())
            .for_each(|q| {
                *count.entry(q).or_insert(0) += 1;
            });
        let safety_factor = count.values().product::<usize>();
        if safety_factor < min_safety_factor {
            min_safety_factor = safety_factor;
            min_safety_factor_step = i;
            min_safety_factor_grid = robots.clone();
        }
    }

    // print_grid(&min_safety_factor_grid);
    Ok(min_safety_factor_step.to_string())
}

fn print_grid(robots: &[Robot]) {
    let mut grid = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }
    for row in grid {
        println!(
            "{}",
            row.iter()
                .map(|i| if *i == 0 {
                    ".".to_string()
                } else {
                    format!("{}", i)
                })
                .collect::<String>()
        );
    }
}

fn parse_one(input: &str) -> IResult<&str, Robot> {
    let (input, _) = tag("p=")(input)?;
    let (input, pos) = map(
        separated_pair(complete::i32, tag(","), complete::i32),
        |(x, y)| IVec2::new(x, y),
    )(input)?;
    let (input, _) = tag(" v=")(input)?;
    let (input, vel) = map(
        separated_pair(complete::i32, tag(","), complete::i32),
        |(x, y)| IVec2::new(x, y),
    )(input)?;
    Ok((input, Robot { pos, vel }))
}

fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(line_ending, parse_one)(input)
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

impl Robot {
    fn step(&mut self) {
        self.pos += self.vel;
        self.pos = self.pos.rem_euclid(IVec2::new(WIDTH, HEIGHT));
    }

    fn quadrant(&self) -> Option<u8> {
        if self.pos.x < WIDTH / 2 && self.pos.y < HEIGHT / 2 {
            Some(0)
        } else if self.pos.x > WIDTH / 2 && self.pos.y < HEIGHT / 2 {
            Some(1)
        } else if self.pos.x < WIDTH / 2 && self.pos.y > HEIGHT / 2 {
            Some(2)
        } else if self.pos.x > WIDTH / 2 && self.pos.y > HEIGHT / 2 {
            Some(3)
        } else {
            None
        }
    }
}
