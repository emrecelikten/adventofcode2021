use std::collections::{BTreeSet, VecDeque};
use std::fmt::Debug;

pub fn dfs<'a, State, CheckF, ExpandF>(
    state: State,
    check_solution: CheckF,
    expand_states: ExpandF,
) -> Option<State>
where
    State: Clone + Ord + Eq + Debug + 'a,
    CheckF: Fn(&State) -> bool,
    ExpandF: Fn(&State) -> Vec<State>,
{
    let mut state_stack: VecDeque<State> = VecDeque::new();
    let mut visited: BTreeSet<State> = BTreeSet::new();
    state_stack.push_front(state);

    while !state_stack.is_empty() {
        let cur_state = state_stack.pop_front().unwrap();
        if check_solution(&cur_state) {
            return Some(cur_state);
        }

        visited.insert(cur_state.clone());

        for state in expand_states(&cur_state) {
            if !visited.contains(&state) {
                state_stack.push_front(state);
            }
        }
    }

    None
}
