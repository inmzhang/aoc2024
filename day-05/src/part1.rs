use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use miette::miette;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (sec1, sec2) = input
        .split("\n\n")
        .next_tuple()
        .ok_or(miette!("parse failed"))?;
    let mut is_after: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in sec1.lines() {
        let (n1, n2) = line
            .split('|')
            .next_tuple()
            .map(|(s1, s2)| (s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()))
            .ok_or(miette!("parse failed"))?;
        is_after.entry(n2).or_default().insert(n1);
    }
    let mut res = 0;
    for line in sec2.lines() {
        let entries = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut valid = true;
        for i in 0..entries.len() - 1 {
            if let Some(before_i) = is_after.get(&entries[i]) {
                if entries[i + 1..entries.len()]
                    .iter()
                    .any(|&x| before_i.contains(&x))
                {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            res += entries[entries.len() / 2];
        }
    }
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
