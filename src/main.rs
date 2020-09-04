use language::Dfa;

fn main() {
    let (mut dfa, start_state_handle) = Dfa::new();

	let a = dfa.new_state();
	let b = dfa.new_state();
	let c = dfa.new_state();

    dfa.add_transition(start_state_handle, a, 'a');
    dfa.add_transition(a, b, 'b');
    dfa.add_transition(b, a, 'c');

	dfa.add_accepting_state(a);

	println!("{}", dfa.try_match("a".chars()));
}
