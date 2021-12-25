use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    positions: [u8; 2],
    scores: [u32; 2],
}

impl State {
    fn with_pos(p1: u8, p2: u8) -> Self {
        State {
            positions: [p1, p2],
            scores: [0, 0],
        }
    }
}

#[derive(Default, Debug)]
struct DeterministicDice {
    last: u32,
    num_rolls: u32,
}

impl DeterministicDice {
    fn roll(&mut self) -> u32 {
        self.last += 1;
        if self.last > 100 {
            self.last = 1;
        }
        self.num_rolls += 1;
        self.last
    }
}

fn play_once(state: &mut State, dice: &mut DeterministicDice) {
    for player in 0..=1 {
        let moves: u32 = (0..3).map(|_| dice.roll()).sum();
        state.positions[player] += (moves % 10) as u8;
        if state.positions[player] > 10 {
            state.positions[player] -= 10;
        }
        state.scores[player] += state.positions[player] as u32;
        if state.scores[player] >= 1000 {
            return;
        }
    }
}

fn play_until_end(state: &mut State, dice: &mut DeterministicDice) {
    while state.scores.iter().all(|&score| score < 1000) {
        play_once(state, dice)
    }
}

fn count_universes_per_dice_sum() -> BTreeMap<u8, u8> {
    let mut result = BTreeMap::new();
    // We enumerate all results of 3 dice rolls and count how many times their sums occur
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                result
                    .entry(i + j + k)
                    .and_modify(|count| {
                        *count += 1;
                    })
                    .or_insert(1);
            }
        }
    }
    result
}

fn play_until_end_dirac(initial_state: &State, universes_per_sum: &BTreeMap<u8, u8>) -> [u64; 2] {
    let mut winning_universes = [0, 0];
    let mut to_visit: VecDeque<(State, u64)> = VecDeque::new();

    to_visit.push_front((initial_state.clone(), 1));

    while let Some(cur) = to_visit.pop_back() {
        let (state, num_universes) = cur;
        for moves1 in 3..=9 {
            let mut new_position_1 = state.positions[0] + moves1;
            if new_position_1 > 10 {
                new_position_1 -= 10;
            }
            let new_score_1 = state.scores[0] + new_position_1 as u32;
            // Skip second player if the first wins
            if new_score_1 >= 21 {
                let num_new_universes = num_universes * universes_per_sum[&moves1] as u64;
                winning_universes[0] += num_new_universes;
                continue;
            }

            for moves2 in 3..=9 {
                let mut new_position_2 = state.positions[1] + moves2;
                if new_position_2 > 10 {
                    new_position_2 -= 10;
                }
                let new_score_2 = state.scores[1] + new_position_2 as u32;
                let num_new_universes = num_universes
                    * universes_per_sum[&moves1] as u64
                    * universes_per_sum[&moves2] as u64;
                if new_score_2 >= 21 {
                    winning_universes[1] += num_new_universes;
                } else {
                    let new_state = State {
                        positions: [new_position_1, new_position_2],
                        scores: [new_score_1, new_score_2],
                    };
                    to_visit.push_front((new_state, num_new_universes));
                }
            }
        }
    }
    winning_universes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_once() {
        let mut state = State::with_pos(4, 8);
        let mut dice = DeterministicDice::default();
        play_once(&mut state, &mut dice);
        assert_eq!(state.positions, [10, 3]);
        assert_eq!(state.scores, [10, 3]);

        play_once(&mut state, &mut dice);
        assert_eq!(state.positions, [4, 6]);
        assert_eq!(state.scores, [14, 9]);
    }

    #[test]
    fn test_play_until_end() {
        let mut state = State::with_pos(4, 8);
        let mut dice = DeterministicDice::default();
        play_until_end(&mut state, &mut dice);
        assert_eq!(state.scores, [1000, 745]);
        assert_eq!(dice.num_rolls, 993);
    }

    #[test]
    fn test_play_until_end_dirac() {
        let universes_per_sum = count_universes_per_dice_sum();
        let state = State::with_pos(4, 8);
        let winning_universes = play_until_end_dirac(&state, &universes_per_sum);
        assert_eq!(winning_universes, [444356092776315, 341960390180808]);
    }

    #[test]
    fn test_d21() {
        let mut state = State::with_pos(8, 1);
        let mut dice = DeterministicDice::default();
        play_until_end(&mut state, &mut dice);
        let losing_player_score = state.scores.iter().min().unwrap();
        let mul = losing_player_score * dice.num_rolls;
        println!("Day 21 result #1: {}", mul);

        let universes_per_sum = count_universes_per_dice_sum();
        let state = State::with_pos(8, 1);
        let winners = play_until_end_dirac(&state, &universes_per_sum);
        let winning_player_num_universes = winners.iter().max().unwrap();
        println!("Day 21 result #2: {}", winning_player_num_universes);
    }
}
