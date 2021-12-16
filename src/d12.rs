use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Graph = BTreeMap<Cave, BTreeSet<Cave>>;

fn to_cave<S: AsRef<str>>(s: S) -> Cave {
    match s {
        s if s.as_ref() == "start" => Cave::Start,
        s if s.as_ref() == "end" => Cave::End,
        s if s.as_ref().chars().any(|ch| ch.is_uppercase()) => Cave::Large(s.as_ref().to_string()),
        s => Cave::Small(s.as_ref().to_string()),
    }
}

fn parse_lines<S: AsRef<str>>(lines: &[S]) -> Graph {
    let mut map = BTreeMap::new();
    for line in lines {
        let mut splitted = line.as_ref().split('-');
        let n1 = to_cave(splitted.next().unwrap());
        let n2 = to_cave(splitted.next().unwrap());
        map.entry(n1.clone())
            .or_insert_with(BTreeSet::new)
            .insert(n2.clone());
        map.entry(n2).or_insert_with(BTreeSet::new).insert(n1);
    }

    map
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Cave {
    Start,
    Small(String),
    Large(String),
    End,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Path {
    nodes: Vec<Cave>,
}

fn traverse_small_once(graph: &Graph, starting_node: &Cave) -> BTreeSet<Path> {
    let mut path_stack: VecDeque<Path> = VecDeque::new();
    let mut visited: BTreeSet<Path> = BTreeSet::new();

    let initial = Path {
        nodes: vec![starting_node.clone()],
    };
    path_stack.push_front(initial);

    while !path_stack.is_empty() {
        let cur_path = path_stack.pop_front().unwrap();
        let last_node = &cur_path.nodes[cur_path.nodes.len() - 1];

        visited.insert(cur_path.clone());

        if last_node == &Cave::End {
            continue;
        }

        for node in graph.get(last_node).unwrap() {
            // Do not visit small caves more than once
            match node {
                Cave::Start => {
                    continue;
                }
                Cave::Small(_) if cur_path.nodes.contains(node) => {
                    continue;
                }
                _ => {}
            }

            let mut new_path = cur_path.clone();
            new_path.nodes.push(node.clone());
            if !visited.contains(&new_path) {
                path_stack.push_front(new_path);
            }
        }
    }

    visited
        .into_iter()
        .filter(|path| path.nodes.iter().last().unwrap() == &Cave::End)
        .collect()
}

fn traverse_small_twice_once(graph: &Graph, starting_node: &Cave) -> BTreeSet<Path> {
    let mut path_stack: VecDeque<Path> = VecDeque::new();
    let mut visited: BTreeSet<Path> = BTreeSet::new();

    let initial = Path {
        nodes: vec![starting_node.clone()],
    };
    path_stack.push_front(initial);

    while !path_stack.is_empty() {
        let cur_path = path_stack.pop_front().unwrap();
        let last_node = &cur_path.nodes[cur_path.nodes.len() - 1];

        visited.insert(cur_path.clone());

        if last_node == &Cave::End {
            continue;
        }

        'nodes: for node in graph.get(last_node).unwrap() {
            // Only visit a single small cave more than once
            match node {
                Cave::Start => {
                    continue;
                }
                Cave::Small(_) if cur_path.nodes.contains(node) => {
                    for i in 0..cur_path.nodes.len() {
                        if matches!(cur_path.nodes[i], Cave::Small(_)) {
                            for j in i + 1..cur_path.nodes.len() {
                                if matches!(cur_path.nodes[j], Cave::Small(_))
                                    && cur_path.nodes[i] == cur_path.nodes[j]
                                {
                                    continue 'nodes;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            let mut new_path = cur_path.clone();
            new_path.nodes.push(node.clone());

            if !visited.contains(&new_path) {
                path_stack.push_front(new_path);
            }
        }
    }

    visited
        .into_iter()
        .filter(|path| path.nodes.iter().last().unwrap() == &Cave::End)
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::common::file_io;

    use super::*;

    const SMALL_TEST_DATA: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const TEST_DATA: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const LARGE_TEST_DATA: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_parse_lines() {
        let lines: Vec<&str> = SMALL_TEST_DATA.lines().collect();
        let graph = parse_lines(&lines);

        assert_eq!(graph.len(), 6);

        let start_expected = BTreeSet::from([to_cave("A"), to_cave("b")]);
        assert_eq!(graph.get(&Cave::Start), Some(&start_expected));

        let a_expected =
            BTreeSet::from([to_cave("start"), to_cave("b"), to_cave("c"), to_cave("end")]);
        assert_eq!(graph.get(&to_cave("A")), Some(&a_expected));

        let d_expected = BTreeSet::from([to_cave("b")]);
        assert_eq!(graph.get(&to_cave("d")), Some(&d_expected));
    }

    #[test]
    fn test_traverse_small_once() {
        let lines_small: Vec<&str> = SMALL_TEST_DATA.lines().collect();
        let graph_small = parse_lines(&lines_small);

        let paths_small = traverse_small_once(&graph_small, &Cave::Start);
        assert_eq!(paths_small.len(), 10);

        let lines_med: Vec<&str> = TEST_DATA.lines().collect();
        let graph_med = parse_lines(&lines_med);

        let paths_med = traverse_small_once(&graph_med, &Cave::Start);
        assert_eq!(paths_med.len(), 19);

        let lines_large: Vec<&str> = LARGE_TEST_DATA.lines().collect();
        let graph_large = parse_lines(&lines_large);

        let paths_large = traverse_small_once(&graph_large, &Cave::Start);
        assert_eq!(paths_large.len(), 226);
    }

    #[test]
    fn test_traverse_small_twice_once() {
        let lines_small: Vec<&str> = SMALL_TEST_DATA.lines().collect();
        let graph_small = parse_lines(&lines_small);

        let paths_small = traverse_small_twice_once(&graph_small, &Cave::Start);
        assert_eq!(paths_small.len(), 36);

        let lines_med: Vec<&str> = TEST_DATA.lines().collect();
        let graph_med = parse_lines(&lines_med);

        let paths_med = traverse_small_twice_once(&graph_med, &Cave::Start);
        assert_eq!(paths_med.len(), 103);

        let lines_large: Vec<&str> = LARGE_TEST_DATA.lines().collect();
        let graph_large = parse_lines(&lines_large);

        let paths_large = traverse_small_twice_once(&graph_large, &Cave::Start);
        assert_eq!(paths_large.len(), 3509);
    }
    #[test]
    fn test_d12() {
        let data = file_io::read_lines_as_strings("inputs/d12").unwrap();
        let graph = parse_lines(&data);

        let small_once_paths = traverse_small_once(&graph, &Cave::Start);
        println!("Day 12 result #1: {}", small_once_paths.len());

        let small_twice_once_paths = traverse_small_twice_once(&graph, &Cave::Start);
        println!("Day 12 result #2: {}", small_twice_once_paths.len());
    }
}
