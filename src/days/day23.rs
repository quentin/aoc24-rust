use crate::{Solution, SolutionPair};
use petgraph::algo::maximal_cliques;
use petgraph::graph::UnGraph;
use std::collections::{HashMap, HashSet};

fn prepare(input: &str) -> Vec<(String, String)> {
    input
        .split_whitespace()
        .map(|s| {
            let mut it = s.split('-');
            (it.next().unwrap(), it.next().unwrap())
        })
        .map(|(a, b)| (a.into(), b.into()))
        .collect()
}

/// Find cliques of size 3 that contain at least one computer with a name starting with 't'.
///
/// For every edge `(a,b)` with computer `a`'s name starting with `'t'`:
/// - find any edge `(a,c)` such that `(b,c)` is an existing edge.
/// - then `{a,b,c}` is a clique of size 3.
///
fn solve_part1(input: &str) -> usize {
    let edges = prepare(input);
    let mut connected: HashSet<(String, String)> = Default::default();
    for (a, b) in &edges {
        connected.insert((a.to_owned(), b.to_owned()));
        connected.insert((b.to_owned(), a.to_owned()));
    }

    let mut triples: HashSet<[String; 3]> = Default::default();
    for (a, b) in &connected {
        if a.starts_with('t') {
            for (a_prime, c) in &connected {
                if a_prime == a && connected.contains(&(b.to_owned(), c.to_owned())) {
                    let mut elems = vec![a, b, c];
                    elems.sort();
                    triples.insert([
                        elems[0].to_owned(),
                        elems[1].to_owned(),
                        elems[2].to_owned(),
                    ]);
                }
            }
        }
    }

    triples.len()
}

/// Find the maximum clique in the network graph: the largest complete subgraph.
///
/// Use `petgraph`'s `maximal_clique` algorithm.
///
fn solve_part2(input: &str) -> String {
    let edges = prepare(input);

    let mut g = UnGraph::<String, ()>::new_undirected();
    let mut computer_index: HashMap<String, petgraph::graph::NodeIndex> = Default::default();
    for (a, b) in &edges {
        let ka = *computer_index
            .entry(a.to_owned())
            .or_insert_with_key(|name| g.add_node(name.to_owned()));
        let kb = *computer_index
            .entry(b.to_owned())
            .or_insert_with_key(|name| g.add_node(name.to_owned()));
        g.add_edge(ka, kb, ());
    }

    let cliques = maximal_cliques(&g);
    let mut maximal_clique = cliques
        .iter()
        .max_by(|c1, c2| c1.len().cmp(&c2.len()))
        .unwrap()
        .iter()
        .map(|k| {
                g.node_weight(*k).unwrap().to_owned()
        })
        .collect::<Vec<_>>();
    maximal_clique.sort();
    maximal_clique.join(",").to_string()

}

pub fn solve(input: String) -> SolutionPair {
    let sol1 = solve_part1(&input);
    let sol2 = solve_part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), "co,de,ka,ta");
    }
}
