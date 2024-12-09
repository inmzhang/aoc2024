use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut data: Vec<i64> = Vec::new();
    input
        .trim_end()
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .for_each(|(i, mut chunk)| {
            let n_occupied: i64 = chunk.next().unwrap().to_string().parse().unwrap();
            data.extend(std::iter::repeat_n(i as i64, n_occupied as usize));
            if let Some(n_space) = chunk.next() {
                let n_space: i64 = n_space.to_string().parse().unwrap();
                data.extend(std::iter::repeat_n(-1, n_space as usize));
            }
        });
    let mut i = 1;
    while i < data.len() {
        let d = data[i];
        if d != -1 {
            i += 1;
            continue;
        }
        data.swap_remove(i);
        data.truncate(data.iter().rposition(|d| *d != -1).unwrap() + 1);
        i += 1;
    }

    let res = data
        .into_iter()
        .enumerate()
        .map(|(i, d)| i as i64 * d)
        .sum::<i64>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
