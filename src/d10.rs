#![allow(dead_code)]
use std::collections::VecDeque;

const fn matching_delimiter(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        other => other,
    }
}

const fn score_wrong_char(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

const fn score_incomplete_char(ch: char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

#[derive(PartialEq, Debug)]
enum ChunkStatus {
    Ok,
    Corrupted { expected: char, actual: char },
    IncompleteLeft,                  // Left chunks missing
    IncompleteRight(VecDeque<char>), // Right chunks missing
}

fn parse_line<S: AsRef<str>>(line: S) -> ChunkStatus {
    let mut stack: VecDeque<char> = VecDeque::new();
    for ch in line.as_ref().chars() {
        if ch == '(' || ch == '[' || ch == '<' || ch == '{' {
            stack.push_front(ch);
        } else if let Some(stack_ch) = stack.pop_front() {
            if ch != matching_delimiter(stack_ch) {
                return ChunkStatus::Corrupted {
                    expected: matching_delimiter(stack_ch),
                    actual: ch,
                };
            }
        } else {
            return ChunkStatus::IncompleteLeft;
        }
    }

    if stack.is_empty() {
        ChunkStatus::Ok
    } else {
        ChunkStatus::IncompleteRight(stack)
    }
}

fn calculate_error_score(chunk_statuses: &[ChunkStatus]) -> usize {
    chunk_statuses
        .iter()
        .filter_map(|status| match status {
            ChunkStatus::Corrupted {
                expected: _,
                actual,
            } => Some(score_wrong_char(*actual)),
            _ => None,
        })
        .sum()
}

fn calculate_completion_score(chunk_statuses: &[ChunkStatus]) -> usize {
    let mut scores: Vec<usize> = chunk_statuses
        .iter()
        .filter_map(|status| match status {
            ChunkStatus::IncompleteRight(stack) => {
                let score = stack.iter().fold(0, |acc, ch| {
                    acc * 5 + score_incomplete_char(matching_delimiter(*ch))
                });
                Some(score)
            }
            _ => None,
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file_io;

    const TEST_DATA: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("{}"), ChunkStatus::Ok);
        assert_eq!(parse_line("{}}"), ChunkStatus::IncompleteLeft);
        assert_eq!(
            parse_line("{{}"),
            ChunkStatus::IncompleteRight(VecDeque::from(['{']))
        );
        assert_eq!(
            parse_line("{)"),
            ChunkStatus::Corrupted {
                expected: '}',
                actual: ')'
            }
        );
        assert_eq!(
            parse_line("<([]){()}[{}])"),
            ChunkStatus::Corrupted {
                expected: '>',
                actual: ')'
            }
        );
        assert_eq!(
            parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
            ChunkStatus::Corrupted {
                expected: ']',
                actual: '}'
            }
        );
    }

    #[test]
    fn test_calculate_error_score() {
        let chunk_results: Vec<ChunkStatus> = TEST_DATA.lines().map(parse_line).collect();
        let score = calculate_error_score(&chunk_results);
        assert_eq!(score, 26397);
    }

    #[test]
    fn test_calculate_completion_score() {
        let chunk_results: Vec<ChunkStatus> = TEST_DATA.lines().map(parse_line).collect();
        let score = calculate_completion_score(&chunk_results);
        assert_eq!(score, 288957);
    }

    #[test]
    fn test_d10() {
        let data = file_io::read_lines_as_strings("inputs/d10").unwrap();
        let chunk_results: Vec<ChunkStatus> = data.iter().map(parse_line).collect();
        let error_score = calculate_error_score(&chunk_results);
        println!("Day 10 result #1: {}", error_score);

        let completion_score = calculate_completion_score(&chunk_results);
        println!("Day 10 result #2: {}", completion_score);
    }
}
