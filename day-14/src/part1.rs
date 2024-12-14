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
const TIME: usize = 100;
// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;
// const TIME: usize = 100;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut robots) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let mut count: HashMap<u8, usize> = HashMap::new();
    robots.iter_mut().for_each(|robot| {
        for _ in 0..TIME {
            robot.step();
        }
        let quadrant = robot.quadrant();
        if let Some(q) = quadrant {
            *count.entry(q).or_insert(0) += 1;
        }
    });
    let res = count.values().product::<usize>();
    Ok(res.to_string())
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

#[derive(Debug)]
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3";
//         assert_eq!("12", process(input)?);
//         Ok(())
//     }
// }
