use language::{Dfa, Nfa};

fn main() {
    let mut nfa = Nfa::new();

    nfa.add_state(0);
    nfa.add_state(1);
    nfa.add_state(2);
    nfa.add_state(3);

    nfa.add_transition(&0, &1, Some('a'));
    nfa.add_transition(&0, &2, Some('a'));
    nfa.add_transition(&1, &2, Some('a'));

    nfa.set_start_state(&0);
	nfa.add_accepting_state(&2);

    let dfa = nfa.into_dfa();
	println!("{}", dfa.try_accept("a".chars()));
	println!("{}", dfa.try_accept("aaa".chars()));
}
