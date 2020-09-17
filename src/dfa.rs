use std::collections::{HashMap, HashSet};

use std::borrow::Borrow;
use std::hash::Hash;

use std::fmt::Debug;

type StateIndex = usize;

#[derive(Debug)]
pub struct Dfa<S, T>
where
    S: Eq + Hash,
    T: Eq + Hash,
{
    states: HashMap<S, StateIndex>,
    start_state: Option<StateIndex>,
    transitions: HashMap<T, HashMap<StateIndex, StateIndex>>,
    accepting_states: HashSet<StateIndex>,
}

impl<S, T> Dfa<S, T>
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
            .or_insert(HashMap::new())
            .insert(start_state_idx, end_state_idx);
    }

    pub fn try_match<I>(&mut self, items: I) -> bool
    where
        I: IntoIterator<Item = T>,
        T: Debug,
    {
        let mut curr_state_idx = self.start_state.expect("Start state has not been set");

        for item in items {
            if let Some(&next_state_idx) = self
                .transitions
                .get(&item)
                .and_then(|states_transition| states_transition.get(&curr_state_idx))
            {
                curr_state_idx = next_state_idx;
            } else {
                return false;
            }
        }

        self.accepting_states.contains(&curr_state_idx)
    }
}
