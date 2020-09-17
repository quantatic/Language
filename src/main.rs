use language::Dfa;
use language::{Nfa, Transition};

fn main() {
    let mut a: Nfa<u32, char> = Nfa::new();

    a.add_state(0);
    a.add_state(1);

    a.set_start_state(0);
    a.add_accepting_state(1);

    a.add_transition(Transition::Value('0'), 0, 0);
    a.add_transition(Transition::Epsilon, 0, 1);
    a.add_transition(Transition::Value('1'), 1, 1);

    println!("{:#?}", a);

    let matched = a.try_match("011".chars());
    println!("Matched: {}", matched);
}
