use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, char},
    combinator::map,
    multi::{many0, many_till},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let mut res = 0;
    let mut enabled = true;
    for i in instructions {
        match i {
            Instruction::Mul(i1, i2) => {
                if enabled {
                    res += i1 * i2;
                }
            }
            Instruction::DO => {
                enabled = true;
            }
            Instruction::DONOT => {
                enabled = false;
            }
        }
    }
    Ok(res.to_string())
}

#[derive(Debug)]
enum Instruction {
    DO,
    DONOT,
    Mul(u32, u32),
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    preceded(
        tag("mul"),
        delimited(
            char('('),
            map(
                separated_pair(complete::u32, char(','), complete::u32),
                |(i1, i2)| Instruction::Mul(i1, i2),
            ),
            char(')'),
        ),
    )(input)
}

fn parse_do_or_donot(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(tag("do()"), |_| Instruction::DO),
        map(tag("don't()"), |_| Instruction::DONOT),
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(map(
        many_till(anychar, alt((parse_mul, parse_do_or_donot))),
        |(_, i)| i,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
