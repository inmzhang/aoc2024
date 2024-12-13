use glam::I64Vec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, machines) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let tokens = machines.into_iter().filter_map(|m| m.tokens()).sum::<i64>();
    Ok(tokens.to_string())
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = terminated(
        map(
            preceded(
                tag("Button A: X+"),
                separated_pair(complete::i64, tag(", Y+"), complete::i64),
            ),
            |(dx, dy)| I64Vec2::new(dx, dy),
        ),
        line_ending,
    )(input)?;
    let (input, button_b) = terminated(
        map(
            preceded(
                tag("Button B: X+"),
                separated_pair(complete::i64, tag(", Y+"), complete::i64),
            ),
            |(dx, dy)| I64Vec2::new(dx, dy),
        ),
        line_ending,
    )(input)?;
    let (input, prize) = terminated(
        map(
            preceded(
                tag("Prize: X="),
                separated_pair(complete::i64, tag(", Y="), complete::i64),
            ),
            |(dx, dy)| I64Vec2::new(dx, dy),
        ),
        opt(line_ending),
    )(input)?;
    Ok((
        input,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, parse_machine)(input)
}

#[derive(Debug)]
struct Machine {
    button_a: I64Vec2,
    button_b: I64Vec2,
    prize: I64Vec2,
}

impl Machine {
    fn tokens(&self) -> Option<i64> {
        let nom = self.prize.y * self.button_b.x - self.prize.x * self.button_b.y
            + (self.button_b.x - self.button_b.y) * 1e13 as i64;
        let denom = self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y;
        if nom % denom != 0 || nom / denom < 0 {
            return None;
        }
        let a = nom / denom;

        let nom = self.prize.y * self.button_a.x - self.prize.x * self.button_a.y
            + (self.button_a.x - self.button_a.y) * 1e13 as i64;
        let denom = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;
        if nom % denom != 0 || nom / denom < 0 {
            return None;
        }
        let b = nom / denom;
        Some(3 * a + b)
    }
}
