use glam::IVec2;
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
    let tokens = machines.into_iter().filter_map(|m| m.tokens()).sum::<i32>();
    Ok(tokens.to_string())
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, button_a) = terminated(
        map(
            preceded(
                tag("Button A: X+"),
                separated_pair(complete::i32, tag(", Y+"), complete::i32),
            ),
            |(dx, dy)| IVec2::new(dx, dy),
        ),
        line_ending,
    )(input)?;
    let (input, button_b) = terminated(
        map(
            preceded(
                tag("Button B: X+"),
                separated_pair(complete::i32, tag(", Y+"), complete::i32),
            ),
            |(dx, dy)| IVec2::new(dx, dy),
        ),
        line_ending,
    )(input)?;
    let (input, prize) = terminated(
        map(
            preceded(
                tag("Prize: X="),
                separated_pair(complete::i32, tag(", Y="), complete::i32),
            ),
            |(dx, dy)| IVec2::new(dx, dy),
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
    button_a: IVec2,
    button_b: IVec2,
    prize: IVec2,
}

impl Machine {
    fn tokens(&self) -> Option<i32> {
        let nom = self.prize.y * self.button_b.x - self.prize.x * self.button_b.y;
        let denom = self.button_a.y * self.button_b.x - self.button_a.x * self.button_b.y;
        if nom % denom != 0 || nom / denom < 0 {
            return None;
        }
        let a = nom / denom;

        let nom = self.prize.y * self.button_a.x - self.prize.x * self.button_a.y;
        let denom = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;
        if nom % denom != 0 || nom / denom < 0 {
            return None;
        }
        let b = nom / denom;
        Some(3 * a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
