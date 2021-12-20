#[derive(PartialEq, Debug, Clone)]
struct Digit {
    value: u32,
    depth: u8,
}

fn parse_line<S: AsRef<str>>(data: S) -> Vec<Digit> {
    let mut cur_depth = 0;
    let mut result = Vec::new();
    let mut num = None;
    for ch in data.as_ref().chars() {
        match ch {
            '[' => {
                cur_depth += 1;
            }
            ']' => {
                if let Some(n) = num {
                    result.push(Digit {
                        value: n,
                        depth: cur_depth,
                    });
                    num = None;
                }
                cur_depth -= 1;
            }
            digit if digit.is_digit(10) => {
                let d = digit.to_digit(10).unwrap();
                if let Some(n) = &mut num {
                    *n = 10 * *n + d;
                } else {
                    num = Some(d);
                }
            }
            ',' => {
                if let Some(n) = num {
                    result.push(Digit {
                        value: n,
                        depth: cur_depth,
                    });
                    num = None;
                }
            }
            _ => {}
        }
    }
    result
}

fn add(pair: &[Digit], number: &mut Vec<Digit>) {
    number.extend(pair.iter().cloned());
    for n in number.iter_mut() {
        n.depth += 1;
    }
}

fn explode(number: &mut Vec<Digit>) -> bool {
    for i in 0..number.len() {
        if number[i].depth >= 5 {
            if i != 0 {
                number[i - 1].value += number[i].value;
            }
            if i + 2 < number.len() {
                number[i + 2].value += number[i + 1].value;
            }
            number[i].depth -= 1;
            number[i].value = 0;
            number.remove(i + 1);
            return true;
        }
    }
    false
}

fn split(number: &mut Vec<Digit>) -> bool {
    for i in 0..number.len() {
        if number[i].value >= 10 {
            let div = number[i].value as f64 / 2.0;
            let left = div.floor() as u32;
            let right = div.ceil() as u32;
            number[i].depth += 1;
            number[i].value = left;
            number.insert(
                i + 1,
                Digit {
                    value: right,
                    depth: number[i].depth,
                },
            );
            return true;
        }
    }
    false
}

fn add_line_to_number(pair: &[Digit], number: &mut Vec<Digit>) {
    add(pair, number);
    loop {
        if explode(number) {
            continue;
        }
        if split(number) {
            continue;
        }
        break;
    }
}

fn calculate_magnitude(number: &[Digit]) -> u32 {
    let mut temp = number.to_vec();
    // We know that the max depth is 4
    let mut depth = 4;
    while depth > 0 {
        let mut processed = false;
        for i in 0..temp.len() - 1 {
            if temp[i].depth == depth && temp[i + 1].depth == depth {
                let sum = 3 * temp[i].value + (2 * temp[i + 1].value);
                temp[i].value = sum;
                temp[i].depth -= 1;
                temp.remove(i + 1);
                processed = true;
                break;
            }
        }
        if !processed {
            depth -= 1;
        }
    }
    temp[0].value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file_io;

    #[test]
    fn test_parse_chars() {
        let data1 = "[[[[1,2],[13,4]],[[5,6],[7,8]]],9]";
        let expected1 = vec![
            Digit { value: 1, depth: 4 },
            Digit { value: 2, depth: 4 },
            Digit {
                value: 13,
                depth: 4,
            },
            Digit { value: 4, depth: 4 },
            Digit { value: 5, depth: 4 },
            Digit { value: 6, depth: 4 },
            Digit { value: 7, depth: 4 },
            Digit { value: 8, depth: 4 },
            Digit { value: 9, depth: 1 },
        ];

        assert_eq!(parse_line(data1), expected1);
    }

    #[test]
    fn test_add() {
        let to_add = parse_line("[3,5]");

        let mut data1 = parse_line("[[1,9],[8,5]]");
        let expected1 = vec![
            Digit { value: 1, depth: 3 },
            Digit { value: 9, depth: 3 },
            Digit { value: 8, depth: 3 },
            Digit { value: 5, depth: 3 },
            Digit { value: 3, depth: 2 },
            Digit { value: 5, depth: 2 },
        ];

        add(&to_add, &mut data1);
        assert_eq!(data1, expected1);

        let mut data2 = parse_line("[9,[8,7]]");
        let expected2 = vec![
            Digit { value: 9, depth: 2 },
            Digit { value: 8, depth: 3 },
            Digit { value: 7, depth: 3 },
            Digit { value: 3, depth: 2 },
            Digit { value: 5, depth: 2 },
        ];

        add(&to_add, &mut data2);
        assert_eq!(data2, expected2);
    }

    #[test]
    fn test_explode() {
        fn helper((data_str, expected_str): (&str, &str)) {
            let mut data = parse_line(data_str);
            explode(&mut data);
            let expected = parse_line(expected_str);
            assert_eq!(data, expected);
        }

        let test_data = vec![
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            // Negative tests
            (
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            ),
        ];

        test_data.into_iter().for_each(helper);
    }

    #[test]
    fn test_split() {
        fn helper((data_str, expected_str): (&str, &str)) {
            let mut data = parse_line(data_str);
            split(&mut data);
            let expected = parse_line(expected_str);
            assert_eq!(data, expected);
        }

        let test_data = vec![
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
            // Negative tests
            (
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];

        test_data.into_iter().for_each(helper);
    }

    #[test]
    fn test_add_line_to_number() {
        fn helper((data_str, to_add_str, expected_str): (&str, &str, &str)) {
            let mut data = parse_line(data_str);
            let to_add = parse_line(to_add_str);
            add_line_to_number(&to_add, &mut data);
            let expected = parse_line(expected_str);
            assert_eq!(data, expected);
        }

        vec![
            (
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            ),
            (
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            ),
            (
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            ),
            (
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            ),
            (
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "[2,9]",
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            ),
            (
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            ),
            (
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            ),
            (
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "[[[[4,2],2],6],[8,7]]",
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ]
        .into_iter()
        .for_each(helper);
    }

    #[test]
    fn test_calculate_magnitude() {
        fn helper((data_str, expected): (&str, u32)) {
            let data = parse_line(data_str);
            let result = calculate_magnitude(&data);
            assert_eq!(result, expected);
        }

        vec![
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
            (
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
                4140,
            ),
        ]
        .into_iter()
        .for_each(helper);
    }
    #[test]
    fn test_d18() {
        let data = file_io::read_lines_as_strings("inputs/d18").unwrap();
        let numbers: Vec<Vec<Digit>> = data.iter().map(parse_line).collect();
        let mut result = numbers[0].clone();

        for number in numbers.iter().skip(1) {
            add_line_to_number(number, &mut result);
        }

        let magnitude = calculate_magnitude(&result);

        println!("Day 18 result #1: {}", magnitude);

        let mut largest = u32::MIN;
        for i in numbers.iter() {
            for j in numbers.iter() {
                let mut n = i.clone();
                add_line_to_number(j, &mut n);
                let magnitude = calculate_magnitude(&n);
                if magnitude > largest {
                    largest = magnitude;
                }
            }
        }

        println!("Day 18 result #2: {}", largest);
    }
}
