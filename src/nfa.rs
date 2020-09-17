use crate::Dfa;

use std::collections::{HashMap, HashSet};

use std::borrow::Borrow;
use std::hash::Hash;

type StateIndex = usize;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Transition<T> {
    Value(T),
    Epsilon,
}

#[derive(Debug)]
pub struct Nfa<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    states: HashMap<S, StateIndex>,
    start_state: Option<StateIndex>,
    transitions: HashMap<Transition<T>, HashMap<StateIndex, HashSet<StateIndex>>>,
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

    pub fn add_transition<B>(&mut self, transition: Transition<T>, start_state: B, end_state: B)
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
        T: std::fmt::Debug,
    {
        let mut curr_states = HashSet::new();
        curr_states.insert(self.start_state.expect("Start state has not been set"));
        loop {
            let to_add = self.states_after_transition(&curr_states, Transition::Epsilon);

            if curr_states.is_superset(&to_add) {
                break;
            }

            curr_states.extend(&to_add);
        }

        for item in items {
            let mut next_states =
                self.states_after_transition(&curr_states, Transition::Value(item));

            loop {
                let to_add = self.states_after_transition(&next_states, Transition::Epsilon);

                if next_states.is_superset(&to_add) {
                    break;
                }

                next_states.extend(&to_add);
            }

            if !next_states.is_empty() {
                curr_states = next_states;
            } else {
                return false;
            }
        }

        !self.accepting_states.is_disjoint(&curr_states)
    }

    fn states_after_transition(
        &self,
        start_states: &HashSet<StateIndex>,
        transition: Transition<T>,
    ) -> HashSet<StateIndex> {
        self.transitions
            .get(&transition)
            .map_or_else(Default::default, |state_transitions| {
                start_states
                    .into_iter()
                    .filter_map(|state| state_transitions.get(&state))
                    .flatten()
                    .copied()
                    .collect()
            })
    }
}
