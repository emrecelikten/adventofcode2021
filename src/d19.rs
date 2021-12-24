use crate::common::algorithms;
use crate::common::collections::vec3d::{Vec3d, Vec3di};
use std::collections::{BTreeMap, BTreeSet, HashMap};

// Distance between detected beacons are invariant w.r.t. scanner position
// We start by finding these distances per scanner, then we match them to other scanners
// Then we find the offset between scanners by calculating beacon_x_scanner_j - beacon_x_scanner_i
// Then transforming all beacons to a single scanner reference, and deduplicating, we find the counts

// This one was a mess. Should have approached differently and in a more principled manner.

type DistanceMap = HashMap<Vec3di, Vec<(Vec3di, f64)>>;

fn parse_chunks<S: AsRef<str>>(chunk: &[S]) -> Vec<Vec3di> {
    chunk
        .iter()
        .skip(1)
        .map(|line| line.as_ref().parse().unwrap())
        .collect()
}

fn compute_distances(coords: &[Vec3di]) -> DistanceMap {
    let mut distances = DistanceMap::new();
    for &b1 in coords.iter() {
        distances.insert(b1, Vec::new());
        for &b2 in coords.iter() {
            // if b1 == b2 { continue; }
            let dist = b1.distance_l2(&b2);
            distances.get_mut(&b1).unwrap().push((b2, dist));
        }
    }
    distances
}

fn find_common_beacons(
    scanner1_inter_distances: &DistanceMap,
    scanner2_inter_distances: &DistanceMap,
) -> Vec<(Vec3di, Vec3di)> {
    fn find_num_common_in_distance_vector(vec1: &[(Vec3di, f64)], vec2: &[(Vec3di, f64)]) -> usize {
        let mut common = 0;

        for (_, dist1) in vec1 {
            for (_, dist2) in vec2 {
                if (dist1 - dist2).abs() < 0.01 {
                    common += 1;
                }
            }
        }

        common
    }

    let mut result = Vec::new();
    for (coord1, dists1) in scanner1_inter_distances {
        for (coord2, dists2) in scanner2_inter_distances {
            let common = find_num_common_in_distance_vector(dists1, dists2);
            if common >= 12 {
                result.push((*coord1, *coord2));
            }
        }
    }

    result
}

type TransformationArray = [(u8, i8); 3];
fn find_scanner_offsets_from_common_beacons(
    common_beacons: &[(Vec3di, Vec3di)],
) -> (Vec3di, TransformationArray, Vec3di, TransformationArray) {
    let (beacon0_scanner1, beacon0_scanner2) = common_beacons[0];
    let (beacon1_scanner1, beacon1_scanner2) = common_beacons[1];

    // Walking from beacon0 to beacon1 should be the same for both scanners, with axes and signs flipped.
    let scanner1_delta = beacon1_scanner1 - beacon0_scanner1;
    let scanner2_delta = beacon1_scanner2 - beacon0_scanner2;

    // We find the axes with same absolute difference, extract the map (e.g. x -> -z, y -> -y) from there
    let mut scanner2_seen_by_scanner1 = Vec3di::default();
    let mut scanner1_seen_by_scanner2 = Vec3di::default();
    let mut mapping_1_to_2 = [(0u8, 0i8); 3];
    let mut mapping_2_to_1 = [(0u8, 0i8); 3];

    for scanner1_coord in 0..3 {
        for scanner2_coord in 0..3 {
            if scanner1_delta[scanner1_coord].abs() == scanner2_delta[scanner2_coord].abs() {
                let sign = scanner1_delta[scanner1_coord].signum()
                    * scanner2_delta[scanner2_coord].signum();
                scanner2_seen_by_scanner1[scanner1_coord] =
                    beacon0_scanner1[scanner1_coord] - beacon0_scanner2[scanner2_coord] * sign;
                scanner1_seen_by_scanner2[scanner2_coord] =
                    beacon0_scanner2[scanner2_coord] - beacon0_scanner1[scanner1_coord] * sign;
                mapping_1_to_2[scanner1_coord as usize] = (scanner2_coord as u8, sign as i8);
                mapping_2_to_1[scanner2_coord as usize] = (scanner1_coord as u8, sign as i8);
            }
        }
    }

    if mapping_1_to_2.iter().any(|&(_, sign)| sign == 0) {
        panic!("Unassigned mapping!");
    }

    (
        scanner2_seen_by_scanner1,
        mapping_1_to_2,
        scanner1_seen_by_scanner2,
        mapping_2_to_1,
    )
}

// Transforms a given vector to another reference
fn transform_vec(v: &Vec3di, mapping: &[(u8, i8); 3]) -> Vec3di {
    let mut result = Vec3di::default();

    for (i, &(pos, sign)) in mapping.iter().enumerate() {
        result[pos as usize] = v[i] * sign as i64;
    }
    result
}

fn find_path(from: usize, transformations: &BTreeMap<(usize, usize), [(u8, i8); 3]>) -> Vec<usize> {
    let check_fn = |state: &Vec<usize>| state[state.len() - 1] == 0;
    let expand_fn = |state: &Vec<usize>| {
        let last = state[state.len() - 1];
        transformations
            .keys()
            .filter_map(|(f, t)| {
                if !state.contains(t) && *f == last {
                    let mut new_state = state.clone();
                    new_state.push(*t);
                    Some(new_state)
                } else {
                    None
                }
            })
            .collect()
    };
    algorithms::search::dfs(vec![from], check_fn, expand_fn).unwrap()
}

type ScannerOffsetMap = BTreeMap<(usize, usize), Vec3di>;
type TransformationMap = BTreeMap<(usize, usize), TransformationArray>;

fn find_all_scanner_offsets(
    beacon_interdistances_per_scanner: &[DistanceMap],
) -> (ScannerOffsetMap, TransformationMap, Vec<Vec<usize>>) {
    let mut offsets: BTreeMap<(usize, usize), Vec3di> = BTreeMap::new();
    let mut transformations: BTreeMap<(usize, usize), [(u8, i8); 3]> = BTreeMap::new();

    for i in 0..beacon_interdistances_per_scanner.len() - 1 {
        for j in (i + 1)..beacon_interdistances_per_scanner.len() {
            let common = find_common_beacons(
                &beacon_interdistances_per_scanner[i],
                &beacon_interdistances_per_scanner[j],
            );
            if !common.is_empty() {
                let (j_by_i, mapping_ij, i_by_j, mapping_ji) =
                    find_scanner_offsets_from_common_beacons(&common);
                offsets.insert((i, j), j_by_i);
                offsets.insert((j, i), i_by_j);
                transformations.insert((i, j), mapping_ij);
                transformations.insert((j, i), mapping_ji);
            }
        }
    }

    // Fill the holes by finding the offset of i-j by using paths i-m-j
    let paths: Vec<Vec<usize>> = (1..beacon_interdistances_per_scanner.len())
        .map(|from| find_path(from, &transformations))
        .collect();

    for path in &paths {
        let edges = path.windows(2);
        let j = path[0];
        for edge in edges {
            let m = edge[0];
            let i = edge[1];
            if offsets.contains_key(&(i, j)) {
                // We already did this
                continue;
            }
            let im = offsets.get(&(i, m)).unwrap();
            let mj = offsets.get(&(m, j)).unwrap();
            let mi_map = transformations.get(&(m, i)).unwrap();

            // go from (j->m) to (k->i) by mapping using k->i transformation
            let offset = im + &transform_vec(mj, mi_map);
            offsets.insert((i, j), offset);
        }
    }

    (offsets, transformations, paths)
}

fn find_unique_beacons(
    beacon_positions_per_scanner: &[Vec<Vec3di>],
    offsets: &BTreeMap<(usize, usize), Vec3di>,
    transformations: &BTreeMap<(usize, usize), [(u8, i8); 3]>,
    paths: &[Vec<usize>],
) -> BTreeSet<Vec3di> {
    let mut positions: BTreeSet<Vec3di> = BTreeSet::new();
    positions.extend(&beacon_positions_per_scanner[0]);

    for scanner in 1..beacon_positions_per_scanner.len() {
        let path = &paths[scanner - 1];
        for beacon_seen_by_scanner in &beacon_positions_per_scanner[scanner] {
            let mut cur = *beacon_seen_by_scanner;
            let edges = path.windows(2);
            for edge in edges {
                let from = edge[0];
                let to = edge[1];

                let scanner_offset = offsets.get(&(to, from)).unwrap();
                let transformation = transformations.get(&(from, to)).unwrap();

                cur = &transform_vec(&cur, transformation) + scanner_offset;
            }
            positions.insert(cur);
        }
    }

    positions
}

fn compute_manhattan(offsets: &BTreeMap<(usize, usize), Vec3di>, len: usize) -> u64 {
    let mut highest = f64::MIN;
    for i in 1..len - 1 {
        for j in i + 1..len {
            let dist = (offsets.get(&(0, i)).unwrap() - offsets.get(&(0, j)).unwrap()).norm_l1();
            if dist > highest {
                highest = dist;
            }
        }
    }
    highest as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::file_io;

    #[test]
    fn test_parse_chunk() {
        let data = file_io::read_lines_as_string_groups("inputs/d19_test").unwrap();
        let beacon_positions_per_scanner: Vec<Vec<Vec3di>> =
            data.iter().map(|s| parse_chunks(s)).collect();
        assert_eq!(
            beacon_positions_per_scanner[0][0],
            Vec3di::new(404, -588, -901)
        );

        assert_eq!(
            beacon_positions_per_scanner[0][beacon_positions_per_scanner[0].len() - 1],
            Vec3di::new(459, -707, 401)
        );

        assert_eq!(
            beacon_positions_per_scanner[4][0],
            Vec3di::new(727, 592, 562)
        );
    }

    #[test]
    fn test_find_common_beacons() {
        let data = file_io::read_lines_as_string_groups("inputs/d19_test").unwrap();
        let beacon_positions_per_scanner: Vec<Vec<Vec3di>> =
            data.iter().map(|s| parse_chunks(s)).collect();
        let scanner0_interdistances: DistanceMap =
            compute_distances(&beacon_positions_per_scanner[0]);
        let scanner1_interdistances: DistanceMap =
            compute_distances(&beacon_positions_per_scanner[1]);

        let common_beacons =
            find_common_beacons(&scanner0_interdistances, &scanner1_interdistances);

        let expected_0: BTreeSet<Vec3di> = r"-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347"
            .lines()
            .map(|s| s.parse().unwrap())
            .collect();

        let expected_1: BTreeSet<Vec3di> = r"686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390"
            .lines()
            .map(|s| s.parse().unwrap())
            .collect();

        let (s0_beacons, s1_beacons): (BTreeSet<Vec3di>, BTreeSet<Vec3di>) =
            common_beacons.into_iter().unzip();
        assert_eq!(s0_beacons, expected_0);
        assert_eq!(s1_beacons, expected_1);
    }

    #[test]
    fn test_transform_vec() {
        let v1_to_v2 = Vec3di::new(3, -1, 2);
        let v2_to_v1 = Vec3di::new(1, 2, 3);
        let ref_1to2 = [(2, -1), (0, 1), (1, -1)]; // x -> -z, y -> x, z -> -y
        let ref_2to1 = [(1, 1), (2, -1), (0, -1)];

        assert_eq!(transform_vec(&v1_to_v2, &ref_1to2), -v2_to_v1);
        assert_eq!(transform_vec(&v2_to_v1, &ref_2to1), -v1_to_v2);

        let beacon = Vec3di::new(3, 4, 5);
        let scanner1 = Vec3di::new(1, 1, 1);
        let scanner2 = Vec3di::new(5, 5, 5);

        let scanner2_from_1 = scanner2 - scanner1;
        let beacon_from_1 = beacon - scanner1;

        let ref_1_to_2 = [(1, -1), (0, -1), (2, -1)];
        let ref_2_to_1 = [(1, -1), (0, -1), (2, -1)];
        let beacon_from_2 = Vec3di::new(1, 2, 0);

        assert_eq!(
            transform_vec(&(beacon_from_1 - scanner2_from_1), &ref_1_to_2),
            beacon_from_2
        );
        assert_eq!(
            scanner2_from_1 + transform_vec(&beacon_from_2, &ref_2_to_1),
            beacon_from_1
        );
    }

    #[test]
    fn test_offsets_and_unique() {
        let data = file_io::read_lines_as_string_groups("inputs/d19_test").unwrap();
        let beacon_positions_per_scanner: Vec<Vec<Vec3di>> =
            data.iter().map(|s| parse_chunks(s)).collect();
        let beacon_interdistances_per_scanner: Vec<DistanceMap> = beacon_positions_per_scanner
            .iter()
            .map(|coords| compute_distances(coords))
            .collect();

        let (offsets, transformations, paths) =
            find_all_scanner_offsets(&beacon_interdistances_per_scanner);

        assert_eq!(offsets.get(&(0, 1)).unwrap(), &Vec3di::new(68, -1246, -43));

        assert_eq!(
            offsets.get(&(0, 2)).unwrap(),
            &Vec3di::new(1105, -1205, 1229)
        );

        assert_eq!(offsets.get(&(0, 3)).unwrap(), &Vec3di::new(-92, -2380, -20));

        assert_eq!(
            offsets.get(&(0, 4)).unwrap(),
            &Vec3di::new(-20, -1133, 1061)
        );

        let unique_beacons = find_unique_beacons(
            &beacon_positions_per_scanner,
            &offsets,
            &transformations,
            &paths,
        );
        assert_eq!(unique_beacons.len(), 79);

        assert_eq!(compute_manhattan(&offsets, data.len()), 3621);
    }

    #[test]
    fn test_d19() {
        let data = file_io::read_lines_as_string_groups("inputs/d19").unwrap();
        let beacon_positions_per_scanner: Vec<Vec<Vec3di>> =
            data.iter().map(|s| parse_chunks(s)).collect();
        let beacon_interdistances_per_scanner: Vec<DistanceMap> = beacon_positions_per_scanner
            .iter()
            .map(|coords| compute_distances(coords))
            .collect();

        let (offsets, transformations, paths) =
            find_all_scanner_offsets(&beacon_interdistances_per_scanner);

        let unique_beacons = find_unique_beacons(
            &beacon_positions_per_scanner,
            &offsets,
            &transformations,
            &paths,
        );

        println!("Day 19 result #1: {}", unique_beacons.len());

        let distance = compute_manhattan(&offsets, data.len());
        println!("Day 19 result #2: {}", distance);
    }
}
