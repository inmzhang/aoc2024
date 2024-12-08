use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut antennas: HashMap<char, Vec<IVec2>> = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antennas
                .entry(c)
                .or_default()
                .push(IVec2::new(row as i32, col as i32));
        }
    }
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let antinodes = antennas
        .into_iter()
        .fold(HashSet::new(), |mut acc, (_, points)| {
            points.iter().tuple_combinations().for_each(|(p1, p2)| {
                let antinode1 = 2 * p1 - p2;
                let antinode2 = 2 * p2 - p1;
                if in_bound(antinode1, width, height) {
                    acc.insert(antinode1);
                }
                if in_bound(antinode2, width, height) {
                    acc.insert(antinode2);
                }
            });
            acc
        });
    Ok(antinodes.len().to_string())
}

fn in_bound(p: IVec2, width: i32, height: i32) -> bool {
    p.x >= 0 && p.x < width && p.y >= 0 && p.y < height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
