use language::Dfa;

fn main() {
    let mut a = Dfa::new();

    a.add_state('0');
    a.add_state('1');

    a.add_transition('0', '0', '0');
    a.add_transition('1', '0', '1');

    a.set_start_state('0');
    a.add_accepting_state('1');
    a.add_accepting_state('1');

    println!("{:#?}", a);

    let matched = a.try_match("000000000001".chars());
    println!("Found match: {}", matched);
}
