#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'X' => 0,
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let d = data[i][j];
            if d != 0 {
                continue;
            }
            count += [
                [0, -1],
                [0, 1],
                [1, -1],
                [1, 1],
                [-1, -1],
                [-1, 1],
                [1, 0],
                [-1, 0],
            ]
            .into_iter()
            .filter(|delta| check_along([i as i32, j as i32], delta, &data))
            .count();
        }
    }
    Ok(count.to_string())
}

fn check_along(mut pos: [i32; 2], delta: &[i32; 2], data: &[Vec<i32>]) -> bool {
    for i in 1..4 {
        pos[0] += delta[0];
        pos[1] += delta[1];
        if pos[0] < 0 || pos[0] >= data.len() as i32 || pos[1] < 0 || pos[1] >= data[0].len() as i32
        {
            return false;
        }
        if data[pos[0] as usize][pos[1] as usize] != i {
            return false;
        }
    }
    true
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
