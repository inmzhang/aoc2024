use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let stones = input
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut counter = HashMap::new();
    for stone in stones {
        counter.insert(stone, 1);
    }
    for _ in 1..=25 {
        blink(&mut counter);
    }

    Ok(counter.values().sum::<usize>().to_string())
}

#[inline]
fn split_int(n: u64) -> (u64, u64) {
    let n_str = n.to_string();
    let uphalf = n_str[0..n_str.len() / 2].parse::<u64>().unwrap();
    let bottomhalf = n_str[n_str.len() / 2..].parse::<u64>().unwrap();
    (uphalf, bottomhalf)
}

fn blink(counter: &mut HashMap<u64, usize>) {
    let mut new_counter = HashMap::with_capacity(counter.capacity());

    for (&stone, count) in counter.iter() {
        let ndigits = stone.checked_ilog10().unwrap_or(0) + 1;
        if stone == 0 {
            *new_counter.entry(1).or_insert(0) += count;
        } else if ndigits % 2 == 0 {
            let (uphalf, bottomhalf) = split_int(stone);
            *new_counter.entry(uphalf).or_insert(0) += count;
            *new_counter.entry(bottomhalf).or_insert(0) += count;
        } else if ndigits % 2 != 0 {
            let n = stone * 2024;
            *new_counter.entry(n).or_insert(0) += count;
        }
    }

    *counter = new_counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
