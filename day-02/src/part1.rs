use miette::miette;
use nom::{
    character::complete::{self, newline, space1},
    combinator::{iterator, opt},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let res = reports
        .into_iter()
        .filter(|report| {
            let is_increasing = (report.last().unwrap() - report.first().unwrap()).signum();
            if is_increasing == 0 {
                return false;
            }
            report.windows(2).all(|w| {
                let diff = w[1] - w[0];
                diff.signum() == is_increasing && diff.abs() <= 3
            })
        })
        .count();

    Ok(res.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let mut it = iterator(
        input,
        terminated(separated_list1(space1, complete::i32), opt(newline)),
    );
    let reports = it.collect::<Vec<_>>();
    let res = it.finish();
    res.map(|(input, _)| (input, reports))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
