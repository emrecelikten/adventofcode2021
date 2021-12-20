use std::collections::BTreeSet;

#[derive(Debug)]
struct Input {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

fn parse_input<S: AsRef<str>>(data: S) -> Input {
    let mut iter = data.as_ref().trim()[13..].split(' ');

    let x_str = iter.next().unwrap();
    let mut x_iter = x_str[2..x_str.len() - 1].split("..");
    let x_min = x_iter.next().unwrap().parse().unwrap();
    let x_max = x_iter.next().unwrap().parse().unwrap();

    let y_str = iter.next().unwrap();
    let mut y_iter = y_str[2..].split("..");
    let y_min = y_iter.next().unwrap().parse().unwrap();
    let y_max = y_iter.next().unwrap().parse().unwrap();

    Input {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

// Since movement on the y axis is symmetrical, we can treat throwing up with v0_y the same as
// throwing down with v0_y from the initial position.
// We need to max v_0 such that there exists an n (discrete time) in natural numbers for
// y_min <= v0_y * n - sum_n(g * n) <= y_max while falling (g=1)
// Using Gauss's summation gives us
// y_min <= v0_y * n - n * (n+1) <= y_max
// Then do the same for x axis
fn find_valid_v0_ys(y_min: i64, y_max: i64) -> Vec<(i64, i64)> {
    let mut v0_y = y_min;
    let mut suitable_v0_ys = Vec::new();

    while -v0_y >= y_min {
        let mut n = 0;
        let mut distance_y = 0;
        while y_min <= distance_y {
            distance_y = (v0_y * (n + 1)) - ((n * (n + 1)) / 2);
            n += 1;

            // Time taken is n downwards and 2*v0_y upwards (goes up and down)
            if distance_y >= y_min && distance_y <= y_max {
                suitable_v0_ys.push((v0_y, n));
            }
        }

        v0_y += 1;
    }

    suitable_v0_ys
}

// Now that we have a list of suitable v0_y values, we do the same for x
// For each v0_y candidate and time taken n, we try to find a valid v_0x
// The only difference is that the v_0x is decreasing each step
fn find_valid_v0_xs(n: i64, x_min: i64, x_max: i64) -> Vec<i64> {
    let mut suitable_v0_xs = Vec::new();
    let mut v0_x = 1;
    let mut distance_x = 0;
    while distance_x <= x_max {
        let lower = std::cmp::max(v0_x - n + 1, 0); // Velocity decrease over time
        distance_x = (v0_x + lower) * (v0_x - lower + 1) / 2; // Sum from lower to v_0x
        if distance_x >= x_min && distance_x <= x_max {
            suitable_v0_xs.push(v0_x);
        }

        v0_x += 1;
    }
    suitable_v0_xs
}

fn find_highest_point(v0_y: i64) -> i64 {
    v0_y * (v0_y + 1) / 2
}

fn find_combinations(ys: &[(i64, i64)], x_min: i64, x_max: i64) -> BTreeSet<(i64, i64)> {
    ys.iter()
        .map(|&(y, n)| {
            find_valid_v0_xs(n, x_min, x_max)
                .iter()
                .map(|&x| (x, y))
                .collect::<Vec<(i64, i64)>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test() {
        let x_min = 20;
        let x_max = 30;
        let y_min = -10;
        let y_max = -5;

        let ys = find_valid_v0_ys(y_min, y_max);
        let &(highest_y, n) = ys.iter().last().unwrap();

        let xs = find_valid_v0_xs(n, x_min, x_max);
        assert_eq!((xs[0], highest_y), (6, 9));

        let highest = find_highest_point(highest_y);
        assert_eq!(highest, 45);

        let combinations = find_combinations(&ys, x_min, x_max);
        assert_eq!(combinations.len(), 112);
    }

    #[test]
    fn test_d17() {
        let data = read_to_string("inputs/d17").unwrap();
        let input = parse_input(&data);

        let ys = find_valid_v0_ys(input.y_min, input.y_max);
        let &(v0_y, _) = ys.iter().last().unwrap();

        let highest_distance = find_highest_point(v0_y);
        println!("Day 17 result #1: {}", highest_distance);

        let combinations = find_combinations(&ys, input.x_min, input.x_max);
        println!("Day 17 result #2: {}", combinations.len());
    }
}
