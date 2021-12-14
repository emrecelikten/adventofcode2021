#![allow(dead_code)]
use crate::common::algorithms::search;
use crate::common::error::CommonError;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct Disp {
    unique_segments: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Disp {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s
            .split(" | ")
            .map(|chunk| chunk.split(' ').map(|e| e.to_string()));

        let unique_segments: Vec<String> = splitted
            .next()
            .ok_or(CommonError::Parse("Cannot parse segment data."))?
            .collect();
        let output: Vec<String> = splitted
            .next()
            .ok_or(CommonError::Parse("Cannot parse segment data."))?
            .collect();

        if unique_segments.len() != 10 || output.len() != 4 {
            Err(CommonError::Parse(
                "Number of chunks for segments and output do not match 10 and 4.",
            ))
        } else {
            Ok(Disp {
                unique_segments,
                output,
            })
        }
    }
}

fn count_unique_outputs(lines: &[Disp]) -> usize {
    lines
        .iter()
        .flat_map(|e| &e.output)
        .filter(|e| e.len() == 2 || e.len() == 4 || e.len() == 3 || e.len() == 7)
        .count()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct State<'a> {
    assignments: BTreeMap<char, char>,
    remaining: BTreeMap<char, BTreeSet<char>>,
    disp: &'a Disp,
}

impl<'a, 'b: 'a> State<'a> {
    fn new(disp: &'b Disp) -> State {
        State {
            assignments: Default::default(),
            remaining: Default::default(),
            disp,
        }
    }
}

impl<'a> Clone for State<'a> {
    fn clone(&self) -> Self {
        State {
            assignments: self.assignments.clone(),
            remaining: self.remaining.clone(),
            disp: self.disp,
        }
    }
}

lazy_static! {
    static ref NUMBERS: BTreeMap<BTreeSet<char>, char> = {
        let mut m = BTreeMap::new();
        m.insert(BTreeSet::from(['a', 'b', 'c', 'e', 'f', 'g']), '0');
        m.insert(BTreeSet::from(['c', 'f']), '1');
        m.insert(BTreeSet::from(['a', 'c', 'd', 'e', 'g']), '2');
        m.insert(BTreeSet::from(['a', 'c', 'd', 'f', 'g']), '3');
        m.insert(BTreeSet::from(['b', 'c', 'd', 'f']), '4');
        m.insert(BTreeSet::from(['a', 'b', 'd', 'f', 'g']), '5');
        m.insert(BTreeSet::from(['a', 'b', 'd', 'e', 'f', 'g']), '6');
        m.insert(BTreeSet::from(['a', 'c', 'f']), '7');
        m.insert(BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']), '8');
        m.insert(BTreeSet::from(['a', 'b', 'c', 'd', 'f', 'g']), '9');
        m
    };
}

fn check_solution<'a, 'b: 'a>(state: &'a State<'b>) -> bool {
    if state.assignments.len() != 7 {
        return false;
    }

    state.disp.unique_segments.iter().all(|segment| {
        let transformed: BTreeSet<char> = segment
            .chars()
            .map(|ch| *state.assignments.get(&ch).unwrap())
            .collect();

        NUMBERS.contains_key(&transformed)
    })
}

fn expand_states<'a, 'b>(state: &'a State<'b>) -> Vec<State<'b>> {
    fn assign_candidate(ch: char, target: char, new_state: &mut State) {
        new_state.assignments.insert(ch, target);
        new_state.remaining.remove(&ch);

        for (_, remaining) in new_state.remaining.iter_mut() {
            remaining.remove(&target);
        }
    }

    // Could do some pruning here
    state
        .remaining
        .iter()
        .filter(|(k, _)| !state.assignments.contains_key(k))
        .flat_map(|(k, values)| {
            values.iter().map(|v| {
                let mut new_state = state.clone();
                assign_candidate(*k, *v, &mut new_state);
                new_state
            })
        })
        .filter(|x| !x.remaining.values().any(|v| v.is_empty()))
        .collect()
}

fn find_map(disp: &Disp) -> Option<BTreeMap<char, char>> {
    let mut initial_state = State::new(disp);

    let mut sorted_segments = disp.unique_segments.clone();
    sorted_segments.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    // Start adding by smaller segments, more restrictive
    for segment in &sorted_segments {
        for ch in segment.chars() {
            if initial_state.remaining.contains_key(&ch) {
                continue;
            }
            let candidates: BTreeSet<char> = NUMBERS
                .keys()
                .filter(|n| n.len() == segment.len())
                .cloned()
                .flatten()
                .collect();

            initial_state.remaining.insert(ch, candidates);
        }
    }

    search::dfs(initial_state, |x| check_solution(x), expand_states).map(|state| state.assignments)
}

fn find_numbers<S: AsRef<str>>(output: &[S], correction_map: &BTreeMap<char, char>) -> Vec<char> {
    output
        .as_ref()
        .iter()
        .map(|s| {
            let transformed: BTreeSet<char> = s
                .as_ref()
                .chars()
                .map(|ch| *correction_map.get(&ch).unwrap())
                .collect();
            *NUMBERS.get(&transformed).unwrap()
        })
        .collect()
}

fn calculate_sum(disps: &[Disp]) -> usize {
    disps.iter().fold(0, |acc, disp| {
        let map = find_map(disp).unwrap();
        let digits: String = find_numbers(&disp.output, &map).iter().collect();
        let num: usize = digits.parse().unwrap();
        acc + num
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};

    const TEST_DATA: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_parse() {
        let line = Disp::from_str(TEST_DATA.lines().next().unwrap()).unwrap();
        assert_eq!(line.unique_segments[0], "be");
        assert_eq!(line.unique_segments[9], "edb");
        assert_eq!(line.output[0], "fdgacbe");
    }

    #[test]
    fn test_count_unique_outputs() {
        let lines = parse::transform_iter(TEST_DATA.lines(), |e| Disp::from_str(e)).unwrap();
        let result = count_unique_outputs(&lines);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_find_map() {
        let lines = parse::transform_iter(std::iter::once("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), |e| Disp::from_str(e)).unwrap();
        let result = find_map(&lines[0]).unwrap();
        let numbers = find_numbers(&lines[0].output, &result);
        assert_eq!(numbers, vec!['5', '3', '5', '3']);
    }

    #[test]
    fn test_calculate_sum() {
        let lines = parse::transform_iter(TEST_DATA.lines(), |e| Disp::from_str(e)).unwrap();
        let sum = calculate_sum(&lines);
        assert_eq!(sum, 61229);
    }

    #[test]
    fn test_d08() {
        let disp_vec: Vec<Disp> = file_io::read_lines_as_structs("inputs/d08").unwrap();

        let count = count_unique_outputs(&disp_vec);
        println!("Day 08 result #1: {}", count);

        let sum = calculate_sum(&disp_vec);
        println!("Day 08 result #2: {}", sum);
    }
}
