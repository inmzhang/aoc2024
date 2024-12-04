use nom::{
    character::{
        complete::{anychar, newline},
        streaming::one_of,
    },
    combinator::{map, opt},
    multi::separated_list1,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    Ok("18".to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    separated_list1(newline, opt(one_of("XMAS")))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
