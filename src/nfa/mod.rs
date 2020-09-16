use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;

use std::fmt::Debug;

use crate::Dfa;

type NfaHandle = usize;

#[derive(Debug)]
pub struct Nfa<S, T>
where
    S: Hash + PartialEq + Eq + Debug,
    T: Hash + PartialEq + Eq + Debug,
{
    start_state: Option<NfaHandle>,
    states: HashMap<S, NfaHandle>,
    accepting_states: HashSet<NfaHandle>,
    transitions: HashMap<Option<T>, HashMap<NfaHandle, BTreeSet<NfaHandle>>>, // transition -> start_state -> end_state
}

impl<S, T> Nfa<S, T>
where
    S: Hash + PartialEq + Eq + Debug,
    T: Hash + PartialEq + Eq + Debug,
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

    pub fn add_transition(&mut self, start: &S, end: &S, val: Option<T>) {
        if let (Some(&start_handle), Some(&end_handle)) =
            (self.states.get(start), self.states.get(end))
        {
            self.transitions
                .entry(val)
                .or_insert_with(HashMap::new)
                .entry(start_handle)
                .or_insert_with(BTreeSet::new)
                .insert(end_handle);
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

    pub fn into_dfa(self) -> Dfa<usize, T>
    where
        T: Clone,
    {
        let mut states_to_dfa_key: HashMap<BTreeSet<usize>, usize> = HashMap::new();
        let mut states_sets_to_consider: Vec<BTreeSet<usize>> = Vec::new();
        let mut res_dfa = Dfa::new();

        let start_state_set: BTreeSet<usize> =
            vec![self.start_state.unwrap()].into_iter().collect();
        states_sets_to_consider.push(start_state_set.clone());

        states_to_dfa_key.insert(start_state_set, 0); // insert the first state set as state 0 -> special case
        res_dfa.add_state(0);
        res_dfa.set_start_state(&0);

        while let Some(state_set_considering) = states_sets_to_consider.pop() {
            println!("{:?}", state_set_considering);

            for (transition, transitions_from_state) in self.transitions.iter() {
                let next_to_consider: BTreeSet<usize> = transitions_from_state
                    .iter()
                    .filter_map(|(start_handle, end_handles): (&usize, &BTreeSet<usize>)| {
                        if state_set_considering.contains(start_handle) {
                            Some(end_handles)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .copied()
                    .collect();

                // if we haven't seen this next set before, add it to the queue and store its
                // mapped idx.
                if !states_to_dfa_key.contains_key(&next_to_consider) {
                    states_sets_to_consider.push(next_to_consider.clone());

                    let next_idx = states_to_dfa_key.len();
                    states_to_dfa_key.insert(next_to_consider.clone(), next_idx);
                    res_dfa.add_state(next_idx);

                    // if accepting state in next to consider, set it as accepting in DFA.
                    if next_to_consider
                        .iter()
                        .any(|state| self.accepting_states.contains(state))
                    {
                        res_dfa.add_accepting_state(&next_idx);
                    }
                }

                res_dfa.add_transition(
                    states_to_dfa_key.get(&state_set_considering).unwrap(),
                    states_to_dfa_key.get(&next_to_consider).unwrap(),
                    transition.clone().unwrap(),
                );
            }
        }

        println!("{:?}", res_dfa);
        res_dfa
    }

    /*pub fn try_accept(&self, vals: impl Iterator<Item = T>) -> bool {
        if let Some(ref start_rc) = self.start_state {
            let mut curr_state = start_rc;
            for val in vals {
                if let Some(transitions_with_val) = self.transitions.get(&val) {
                    if let Some(next_state) = transitions_with_val.get(curr_state) {
                        curr_state = next_state;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            self.accepting_states.contains(curr_state)
        } else {
            panic!("No start state set!");
        }
    }*/
}
