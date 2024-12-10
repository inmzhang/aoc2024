#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (data, zero_nodes) = parse(input);
    let counts = zero_nodes
        .iter()
        .map(|node| traverse(node, &data))
        .sum::<usize>();

    Ok(counts.to_string())
}

fn parse(input: &str) -> (Vec<Vec<u8>>, Vec<Node>) {
    let mut zero_nodes = vec![];
    let mut data = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap() as u8;
            row.push(value);
            if value == 0 {
                let node = Node::new(i, j, value);
                zero_nodes.push(node);
            }
        }
        data.push(row);
    }
    (data, zero_nodes)
}

fn traverse(cur: &Node, data: &[Vec<u8>]) -> usize {
    if cur.value == 9 {
        return 1;
    }
    let next_nodes = cur.next(data);
    if next_nodes.is_empty() {
        return 0;
    }
    next_nodes.iter().map(|node| traverse(node, data)).sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node {
    i: usize,
    j: usize,
    value: u8,
}

impl Node {
    fn new(i: usize, j: usize, value: u8) -> Self {
        Self { i, j, value }
    }

    fn next(&self, data: &[Vec<u8>]) -> Vec<Node> {
        let mut nodes = vec![];
        if self.i > 0 && data[self.i - 1][self.j] == self.value + 1 {
            nodes.push(Node::new(self.i - 1, self.j, self.value + 1));
        }
        if self.i < data.len() - 1 && data[self.i + 1][self.j] == self.value + 1 {
            nodes.push(Node::new(self.i + 1, self.j, self.value + 1));
        }
        if self.j > 0 && data[self.i][self.j - 1] == self.value + 1 {
            nodes.push(Node::new(self.i, self.j - 1, self.value + 1));
        }
        if self.j < data[self.i].len() - 1 && data[self.i][self.j + 1] == self.value + 1 {
            nodes.push(Node::new(self.i, self.j + 1, self.value + 1));
        }
        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("81", process(input)?);
        Ok(())
    }
}
