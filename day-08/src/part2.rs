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
                for x in 0..width {
                    let rem = ((x - p1.x) * (p2.y - p1.y)) % (p2.x - p1.x);
                    if rem != 0 {
                        continue;
                    }
                    let antinode =
                        IVec2::new(x, ((x - p1.x) * (p2.y - p1.y)) / (p2.x - p1.x) + p1.y);
                    if in_bound(antinode, width, height) {
                        acc.insert(antinode);
                    }
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
    fn test_process1() -> miette::Result<()> {
        let input = "T........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!("9", process(input)?);
        Ok(())
    }
    #[test]
    fn test_process2() -> miette::Result<()> {
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
        assert_eq!("34", process(input)?);
        Ok(())
    }
}
