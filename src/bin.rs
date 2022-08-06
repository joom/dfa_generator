extern crate dfa_generator;
use dfa_generator::regex::*;
use dfa_generator::re_to_nfa;
use dfa_generator::nfa_to_dfa;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Step {
    A,
    B,
    C,
    D,
}

pub fn main() {
    let a = Regex::Literal(Step::A);
    let b = Regex::Literal(Step::B);
    let c = Regex::Literal(Step::C);
    let d = Regex::Literal(Step::D);
    let b_or_c = b.alternate(&c);
    let rgx = a.concatenate(&b_or_c).concatenate(&d);
    let nfa = re_to_nfa::converter(rgx);
    let dfa = nfa_to_dfa::converter(nfa);
    println!("{:?}", dfa);
}
