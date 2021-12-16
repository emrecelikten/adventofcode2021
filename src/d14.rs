use std::collections::HashMap;

// Maybe a trie would have been better here
type TransformationMap = HashMap<String, char>;

fn parse_transformations<S: AsRef<str>>(lines: &[S]) -> TransformationMap {
    let mut map = TransformationMap::new();
    for line in lines {
        let splitted: Vec<String> = line.as_ref().split(" -> ").map(String::from).collect();
        assert_eq!(splitted.len(), 2);
        map.insert(splitted[0].clone(), splitted[1].chars().next().unwrap());
    }
    map
}

fn step(polymer: &mut Vec<char>, transformation_map: &TransformationMap) {
    let mut i = 0;
    while i < polymer.len() - 1 {
        let pair: String = [polymer[i], polymer[i + 1]].iter().collect();
        if let Some(resulting_element) = transformation_map.get(&pair) {
            polymer.insert(i + 1, *resulting_element);
            i += 1;
        }
        i += 1;
    }
}

fn count_element_occurrences(polymer: &[char]) -> HashMap<char, usize> {
    let mut occurrence_map = HashMap::new();
    for ch in polymer {
        occurrence_map
            .entry(*ch)
            .and_modify(|e| {
                *e += 1;
            })
            .or_insert(1);
    }

    occurrence_map
}

fn find_min_max(occurrences: HashMap<char, usize>) -> usize {
    let min = occurrences
        .iter()
        .min_by(|&e1, &e2| e1.1.cmp(e2.1))
        .unwrap();
    let max = occurrences
        .iter()
        .max_by(|&e1, &e2| e1.1.cmp(e2.1))
        .unwrap();
    max.1 - min.1
}

fn step_count(
    pair_counts: &mut HashMap<String, usize>,
    element_counts: &mut HashMap<char, usize>,
    transformation_map: &TransformationMap,
) {
    let mut additions = HashMap::new();

    for (pair, pair_count) in pair_counts.drain() {
        if let Some(resulting_element) = transformation_map.get(&pair) {
            let new_pair1: String = [pair.chars().next().unwrap(), *resulting_element]
                .iter()
                .collect();
            let new_pair2: String = [*resulting_element, pair.chars().nth(1).unwrap()]
                .iter()
                .collect();

            additions
                .entry(new_pair1)
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(pair_count);
            additions
                .entry(new_pair2)
                .and_modify(|count| {
                    *count += pair_count;
                })
                .or_insert(pair_count);
            element_counts
                .entry(*resulting_element)
                .and_modify(|count| *count += pair_count)
                .or_insert(pair_count);
        }
    }

    for (pair, new_count) in additions.iter() {
        pair_counts
            .entry(pair.clone())
            .and_modify(|count| {
                *count += new_count;
            })
            .or_insert(*new_count);
    }
}

fn convert_polymer_to_pair_counts(polymer: &[char]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for chs in polymer.windows(2) {
        let s = chs.iter().collect();
        counts
            .entry(s)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{file_io, parse};

    const TEST_DATA: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_parse_transformations() {
        let data = parse::split_per_double_newline(TEST_DATA);
        let map = parse_transformations(&data[1]);
        assert_eq!(map.len(), 16);
        assert_eq!(map.get("CH"), Some(&'B'));
        assert_eq!(map.get("CN"), Some(&'C'));
    }

    #[test]
    fn test_step() {
        let data = parse::split_per_double_newline(TEST_DATA);
        let mut polymer = data[0][0].chars().collect();
        let map = parse_transformations(&data[1]);

        step(&mut polymer, &map);
        assert_eq!(polymer, "NCNBCHB".chars().collect::<Vec<char>>());

        step(&mut polymer, &map);
        assert_eq!(polymer, "NBCCNBBBCBHCB".chars().collect::<Vec<char>>());

        step(&mut polymer, &map);
        assert_eq!(
            polymer,
            "NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>()
        );

        step(&mut polymer, &map);
        assert_eq!(
            polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .chars()
                .collect::<Vec<char>>()
        );

        step(&mut polymer, &map);
        assert_eq!(polymer.len(), 97);
    }

    #[test]
    fn test_count_occurrences() {
        let data = parse::split_per_double_newline(TEST_DATA);
        let mut polymer = data[0][0].chars().collect();
        let map = parse_transformations(&data[1]);

        for _ in 0..10 {
            step(&mut polymer, &map);
        }

        let occurrences = count_element_occurrences(&polymer);
        assert_eq!(occurrences.get(&'B'), Some(&1749));
        assert_eq!(occurrences.get(&'C'), Some(&298));
        assert_eq!(occurrences.get(&'H'), Some(&161));
        assert_eq!(occurrences.get(&'N'), Some(&865));
    }

    #[test]
    fn test_step_count() {
        let data = parse::split_per_double_newline(TEST_DATA);
        let polymer: Vec<char> = data[0][0].chars().collect();
        let map = parse_transformations(&data[1]);
        let mut polymer_pair_counts = convert_polymer_to_pair_counts(&polymer);
        let mut element_counts = count_element_occurrences(&polymer);

        step_count(&mut polymer_pair_counts, &mut element_counts, &map);
        let expected_1 = convert_polymer_to_pair_counts(&"NCNBCHB".chars().collect::<Vec<char>>());
        assert_eq!(polymer_pair_counts, expected_1);

        step_count(&mut polymer_pair_counts, &mut element_counts, &map);
        let expected_2 =
            convert_polymer_to_pair_counts(&"NBCCNBBBCBHCB".chars().collect::<Vec<char>>());
        assert_eq!(polymer_pair_counts, expected_2);

        step_count(&mut polymer_pair_counts, &mut element_counts, &map);
        let expected_3 = convert_polymer_to_pair_counts(
            &"NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>(),
        );
        assert_eq!(polymer_pair_counts, expected_3);

        step_count(&mut polymer_pair_counts, &mut element_counts, &map);
        let polymer_4 = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            .chars()
            .collect::<Vec<char>>();
        let expected_4 = convert_polymer_to_pair_counts(&polymer_4);
        assert_eq!(polymer_pair_counts, expected_4);
        assert_eq!(element_counts, count_element_occurrences(&polymer_4));
    }
    #[test]
    fn test_count_occurrences_from_pairs() {
        let data = parse::split_per_double_newline(TEST_DATA);
        let polymer : Vec<char>= data[0][0].chars().collect();
        let map = parse_transformations(&data[1]);
        let mut polymer_pair_counts = convert_polymer_to_pair_counts(&polymer);
        let mut element_counts = count_element_occurrences(&polymer);
        for _ in 0..10 {
            step_count(&mut polymer_pair_counts, &mut element_counts, &map);
        }

        assert_eq!(element_counts.get(&'B'), Some(&1749));
        assert_eq!(element_counts.get(&'C'), Some(&298));
        assert_eq!(element_counts.get(&'H'), Some(&161));
        assert_eq!(element_counts.get(&'N'), Some(&865));
    }

    #[test]
    fn test_d14() {
        let data = file_io::read_lines_as_string_groups("inputs/d14").unwrap();
        let mut polymer: Vec<char> = data[0][0].chars().collect();
        let map = parse_transformations(&data[1]);
        for _ in 0..10 {
            step(&mut polymer, &map);
        }
        let occurrences_10 = count_element_occurrences(&polymer);
        let diff_10 = find_min_max(occurrences_10);
        println!("Day 14 result #1: {}", diff_10);

        polymer = data[0][0].chars().collect();
        let mut polymer_pair_counts = convert_polymer_to_pair_counts(&polymer);
        let mut occurrences_40 = count_element_occurrences(&polymer);

        for _ in 0..40 {
            step_count(&mut polymer_pair_counts, &mut occurrences_40, &map);
        }

        let diff_40 = find_min_max(occurrences_40);
        println!("Day 14 result #2: {}", diff_40);
    }
}
