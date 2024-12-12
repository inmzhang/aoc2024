use std::collections::HashSet;

use glam::IVec2;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let groups = parse(input);
    let price = groups.iter().fold(0, |acc, group| acc + group.price());
    Ok(price.to_string())
}

fn parse(input: &str) -> Vec<Group> {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut groups: Vec<Group> = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, &kind) in row.iter().enumerate() {
            let pos = IVec2::new(i as i32, j as i32);
            if groups
                .iter()
                .any(|g| g.kind == kind && g.components.contains(&pos))
            {
                continue;
            }
            let mut components: HashSet<IVec2> = HashSet::new();
            components.insert(pos);
            traverse(&data, pos, &mut components);
            groups.push(Group { kind, components });
        }
    }
    groups
}

fn traverse(data: &[Vec<char>], pos: IVec2, visited: &mut HashSet<IVec2>) {
    let kind = data[pos.x as usize][pos.y as usize];
    [
        IVec2::new(0, 1),
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ]
    .iter()
    .for_each(|dir| {
        let new_pos = pos + *dir;
        if !in_bounds(new_pos, data)
            || visited.contains(&new_pos)
            || data[new_pos.x as usize][new_pos.y as usize] != kind
        {
            return;
        }
        visited.insert(new_pos);
        traverse(data, new_pos, visited);
    });
}

fn in_bounds(pos: IVec2, data: &[Vec<char>]) -> bool {
    pos.x >= 0 && pos.x < data.len() as i32 && pos.y >= 0 && pos.y < data[0].len() as i32
}

#[derive(Debug)]
struct Group {
    kind: char,
    components: HashSet<IVec2>,
}

impl Group {
    fn area(&self) -> usize {
        self.components.len()
    }

    fn perimeter(&self) -> usize {
        self.components.iter().fold(0, |acc, &pos| {
            acc + [
                IVec2::new(0, 1),
                IVec2::new(0, -1),
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
            ]
            .iter()
            .filter(|dir| {
                let new_pos = pos + **dir;
                !self.components.contains(&new_pos)
            })
            .count()
        })
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() -> miette::Result<()> {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!("140", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> miette::Result<()> {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!("772", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process3() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
