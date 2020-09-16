use std::collections::{HashMap, HashSet};
use std::hash::Hash;

type DfaHandle = usize;

#[derive(Debug)]
pub struct Dfa<S, T>
where
    S: Hash + PartialEq + Eq,
    T: Hash + PartialEq + Eq,
{
    start_state: Option<DfaHandle>,
    states: HashMap<S, DfaHandle>,
    accepting_states: HashSet<DfaHandle>,
    transitions: HashMap<T, HashMap<DfaHandle, DfaHandle>>, // transition -> start_state -> end_state
}

impl<S, T> Dfa<S, T>
where
    S: Hash + PartialEq + Eq,
    T: Hash + PartialEq + Eq,
{
    pub fn new() -> Self {
        Self {
            start_state: None,
            states: HashMap::new(),
            accepting_states: HashSet::new(),
            transitions: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, state: S) {
        self.states.insert(state, self.states.len());
    }

    pub fn add_transition(&mut self, start: &S, end: &S, val: T) {
        if let (Some(&start_handle), Some(&end_handle)) =
            (self.states.get(start), self.states.get(end))
        {
            self.transitions
                .entry(val)
                .or_insert_with(HashMap::new)
                .insert(start_handle, end_handle);
        } else {
            panic!("Missing start or end state for added transition!")
        };
    }

    pub fn set_start_state(&mut self, start_state: &S) {
        if let Some(&start_handle) = self.states.get(start_state) {
            self.start_state = Some(start_handle);
        } else {
            println!("Attempting to set start state that doesn't exist");
        }
    }

    pub fn add_accepting_state(&mut self, accepting_state: &S) {
        if let Some(&accepting_handle) = self.states.get(accepting_state) {
            self.accepting_states.insert(accepting_handle);
        } else {
            panic!("Attempting to set an accepting state that doesn't exist");
        }
    }

    pub fn try_accept(&self, vals: impl Iterator<Item = T>) -> bool {
        if let Some(start_handle) = self.start_state {
            let mut curr_state_handle = start_handle;
            for val in vals {
                if let Some(transitions_from_state) = self.transitions.get(&val) {
                    if let Some(&next_state_handle) = transitions_from_state.get(&start_handle) {
                        curr_state_handle = next_state_handle;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            self.accepting_states.contains(&curr_state_handle)
        } else {
            panic!("No start state set!");
        }
    }
}
