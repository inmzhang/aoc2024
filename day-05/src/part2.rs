use std::collections::HashMap;

use itertools::Itertools;
use miette::miette;
use petgraph::{algo::toposort, prelude::DiGraphMap};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (sec1, sec2) = input
        .split("\n\n")
        .next_tuple()
        .ok_or(miette!("parse failed"))?;
    let mut graph = DiGraphMap::<u32, ()>::new();
    for line in sec1.lines() {
        let (n1, n2) = line
            .split('|')
            .next_tuple()
            .map(|(s1, s2)| (s1.parse::<u32>().unwrap(), s2.parse::<u32>().unwrap()))
            .ok_or(miette!("parse failed"))?;
        graph.add_edge(n1, n2, ());
    }
    let mut res = 0;
    for line in sec2.lines() {
        let entries = line
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut sub_graph = graph.clone();
        for node in graph.nodes() {
            if !entries.contains(&node) {
                sub_graph.remove_node(node);
            }
        }
        let topo_sort = toposort(&sub_graph, None).map_err(|_| miette!("toposort failed"))?;
        let index_map = topo_sort
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect::<HashMap<_, _>>();
        let mut entries = line
            .split(',')
            .map(|s| *index_map.get(&s.parse::<u32>().unwrap()).unwrap())
            .collect::<Vec<_>>();
        if entries.is_sorted() {
            continue;
        }
        entries.sort();
        res += topo_sort[entries[entries.len() / 2]]
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
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
