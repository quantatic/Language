use crate::Dfa;

use std::collections::{HashMap, HashSet};

use std::borrow::Borrow;
use std::hash::Hash;

type StateIndex = usize;

#[derive(Debug)]
pub struct Nfa<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    states: HashMap<S, StateIndex>,
    start_state: Option<StateIndex>,
    transitions: HashMap<T, HashMap<StateIndex, HashSet<StateIndex>>>,
    accepting_states: HashSet<StateIndex>,
}

impl<S, T> Nfa<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            start_state: None,
            transitions: HashMap::new(),
            accepting_states: HashSet::new(),
        }
    }

    pub fn add_state(&mut self, state: S) {
        self.states.insert(state, self.states.len());
    }

    pub fn set_start_state<B>(&mut self, state: B)
    where
        B: Borrow<S>,
    {
        let start_state_idx = *self
            .states
            .get(state.borrow())
            .expect("Couldn't find start state");

        self.start_state = Some(start_state_idx);
    }

    pub fn add_accepting_state<B>(&mut self, state: B)
    where
        B: Borrow<S>,
    {
        let accepting_state_idx = *self
            .states
            .get(state.borrow())
            .expect("Couldn't find accepting state");

        self.accepting_states.insert(accepting_state_idx);
    }

    pub fn add_transition<B>(&mut self, transition: T, start_state: B, end_state: B)
    where
        B: Borrow<S>,
    {
        let start_state_idx = *self
            .states
            .get(start_state.borrow())
            .expect("Couldn't find start state");

        let end_state_idx = *self
            .states
            .get(end_state.borrow())
            .expect("Couldn't find start state");

        self.transitions
            .entry(transition)
            .or_insert_with(HashMap::new)
            .entry(start_state_idx)
            .or_insert_with(HashSet::new)
            .insert(end_state_idx);
    }

    pub fn build_dfa(self) -> Dfa<S, T> {
        Dfa::new()
    }

    pub fn try_match<I>(&mut self, items: I) -> bool
    where
        I: IntoIterator<Item = T>,
    {
        let mut curr_states = HashSet::new();
        curr_states.insert(self.start_state.expect("Start state has not been set"));

        for item in items {
            println!("{:?}", curr_states);

            if let Some(next_states) = self.transitions.get(&item).map(|state_transitions| {
                curr_states
                    .into_iter()
                    .filter_map(|state| state_transitions.get(&state))
                    .flatten()
                    .copied()
                    .collect()
            }) {
                curr_states = next_states;
            } else {
                return false;
            }
        }

        !self.accepting_states.is_disjoint(&curr_states)
    }
}
