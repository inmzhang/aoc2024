use itertools::{repeat_n, Itertools};
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, equations) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let res = equations
        .into_iter()
        .filter(|(target, operands)| validate_equation(*target, operands))
        .map(|(target, _)| target)
        .sum::<u64>();

    Ok(res.to_string())
}

fn validate_equation(target: u64, operands: &[u64]) -> bool {
    repeat_n(
        [Operator::Add, Operator::Multiply, Operator::Combine].into_iter(),
        operands.len() - 1,
    )
    .multi_cartesian_product()
    .any(|operators| {
        let res = operands[1..]
            .iter()
            .zip(operators.iter())
            .fold(operands[0], |acc, (operand, operator)| {
                operator.apply(acc, *operand)
            });
        res == target
    })
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Combine,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Combine => a * 10_u64.pow(b.ilog10() + 1) + b,
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (input, target) = complete::u64(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, operands) = separated_list1(space1, complete::u64)(input)?;
    Ok((input, (target, operands)))
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(line_ending, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
