use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct Dfa {
    states: HashMap<StateHandle, State>,
	start_state_handle: StateHandle,
	accepting_state_handles: HashSet<StateHandle>
}

impl Dfa {
    pub fn new() -> (Self, StateHandle) {
		let start_state_handle = StateHandle(0);

        let mut res = Self {
            states: HashMap::new(),
			start_state_handle: start_state_handle,
			accepting_state_handles: HashSet::new()
        };

		res.states.insert(StateHandle(0), State {transitions: HashMap::new()});
		(res, start_state_handle)
    }

    pub fn new_state(&mut self) -> StateHandle {
		let new_handle = (0..usize::MAX)
			.map(|idx| StateHandle(idx))
            .find(|handle| !self.states.contains_key(handle))
            .unwrap();

		self.add_empty_state(new_handle);

		new_handle
    }

    pub fn add_transition(&mut self, start: StateHandle, end: StateHandle, val: char) {
        if !self.states.contains_key(&start) {
            self.states.insert(
                start,
                State {
                    transitions: HashMap::new(),
                },
            );
        }

        if !self.states.contains_key(&end) {
            self.states.insert(
                end,
                State {
                    transitions: HashMap::new(),
                },
            );
        }

        let start_node = self.states.get_mut(&start).unwrap();
        if let Some(_) = start_node.transitions.insert(val, end) {
            panic!("Transition: {} already exists!", val);
        }
    }

	pub fn add_accepting_state(&mut self, accepting: StateHandle) {
		self.accepting_state_handles.insert(accepting);
	}

	pub fn try_match<I: IntoIterator<Item=char>>(&self, vals: I) -> bool {
		let mut curr_state_handle = &self.start_state_handle;
		for val in vals {
			println!("state: {:?}", curr_state_handle);
			// If current handle is valid state
			if let Some(curr_state) = self.states.get(curr_state_handle) {
				// If current state has transition on next char
				if let Some(next_state_handle) = curr_state.transitions.get(&val) {
					curr_state_handle = next_state_handle;
				} else {
					return false;
				}
			} else {
				return false;
			}
		}

		self.accepting_state_handles.contains(curr_state_handle)
	}

	fn add_empty_state(&mut self, handle: StateHandle) {
		if let Some(_) = self.states.insert(handle, State {transitions: HashMap::new()}) {
			panic!("Trying to add state with handle: {:?} that already exists!", handle);
		}
	}
}

#[derive(Debug)]
struct State {
    transitions: HashMap<char, StateHandle>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateHandle(usize);
