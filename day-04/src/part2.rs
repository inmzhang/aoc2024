#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'A' => 0,
                    'M' => 10,
                    'S' => 100,
                    'X' => 1000,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let d = data[i][j];
            if d == 0 && check_around([i, j], &data) {
                count += 1;
            }
        }
    }
    Ok(count.to_string())
}

fn check_around(pos: [usize; 2], data: &[Vec<i32>]) -> bool {
    if pos[0] < 1 || pos[0] >= data.len() - 1 || pos[1] < 1 || pos[1] >= data[0].len() - 1 {
        return false;
    }
    let [i, j] = pos;
    data[i - 1][j - 1] + data[i + 1][j + 1] == 110 && data[i - 1][j + 1] + data[i + 1][j - 1] == 110
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
