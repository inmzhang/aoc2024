use glam::IVec2;
use pathfinding::prelude::bfs;

const WIDTH: i32 = 70;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let corrupted = parse(input);
    let start = IVec2::new(0, 0);
    // Binary search
    let mut left = 0;
    let mut right = corrupted.len() - 1;
    while left < right {
        let middle = (left + right) / 2;
        if bfs(
            &start,
            |node| successors(node, &corrupted[..=middle]),
            |n| n == &IVec2::new(WIDTH, WIDTH),
        )
        .is_some()
        {
            left = middle + 1;
        } else {
            right = middle;
        }
    }
    let (x, y) = corrupted[right].into();
    Ok(format!("{},{}", x, y))
}

fn successors(node: &IVec2, corrupted: &[IVec2]) -> Vec<IVec2> {
    let mut result = Vec::new();
    for &dir in &[IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y] {
        let next = *node + dir;
        if next.x >= 0
            && next.x <= WIDTH
            && next.y >= 0
            && next.y <= WIDTH
            && !corrupted.contains(&next)
        {
            result.push(next);
        }
    }
    result
}

fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            IVec2::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = "5,4
// 4,2
// 4,5
// 3,0
// 2,1
// 6,3
// 2,4
// 1,5
// 0,6
// 3,3
// 2,6
// 5,1
// 1,2
// 5,5
// 2,5
// 6,5
// 1,4
// 0,4
// 6,4
// 1,1
// 6,1
// 1,0
// 0,5
// 1,6
// 2,0";
//         assert_eq!("6,1", process(input)?);
//         Ok(())
//     }
// }
