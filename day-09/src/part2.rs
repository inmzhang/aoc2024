use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut data: Vec<i64> = Vec::new();
    let mut occupied: Vec<(usize, usize)> = vec![];
    let mut empty_spaces: Vec<(usize, usize)> = vec![];
    input
        .trim_end()
        .chars()
        .chunks(2)
        .into_iter()
        .enumerate()
        .for_each(|(i, mut chunk)| {
            let n_occupied: i64 = chunk.next().unwrap().to_string().parse().unwrap();
            occupied.push((data.len(), n_occupied as usize));
            data.extend(std::iter::repeat_n(i as i64, n_occupied as usize));
            if let Some(n_space) = chunk.next() {
                let n_space: i64 = n_space.to_string().parse().unwrap();
                if n_space == 0 {
                    return;
                }
                empty_spaces.push((data.len(), n_space as usize));
                data.extend(std::iter::repeat_n(-1, n_space as usize));
            }
        });

    for (i, n) in occupied.into_iter().rev() {
        if let Some(e) = (0..empty_spaces.len()).find(|&e| {
            let (j, s) = empty_spaces[e];
            j < i && s >= n
        }) {
            let (j, s) = empty_spaces[e];
            (0..n).for_each(|k| {
                data.swap(i + k, j + k);
                data[i + k] = -1;
            });
            empty_spaces[e] = (j + n, s - n);
        }
    }

    let res = data
        .into_iter()
        .enumerate()
        .map(|(i, d)| if d != -1 { i as i64 * d } else { 0 })
        .sum::<i64>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
