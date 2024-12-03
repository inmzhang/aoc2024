use miette::miette;
use nom::{
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
    let res = instructions
        .into_iter()
        .map(|Instruction::Mul(i1, i2)| i1 * i2)
        .sum::<u32>();
    Ok(res.to_string())
}

#[derive(Debug)]
enum Instruction {
    Mul(u32, u32),
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(map(
        many_till(
            anychar,
            preceded(
                tag("mul"),
                delimited(
                    char('('),
                    separated_pair(complete::u32, char(','), complete::u32),
                    char(')'),
                ),
            ),
        ),
        |(_, (i1, i2))| Instruction::Mul(i1, i2),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
