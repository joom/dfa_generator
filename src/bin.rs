extern crate dfa_generator;
use dfa_generator::regex::*;

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
    let dfa = rgx.to_nfa().to_dfa();
    println!("{:?}", dfa.graph);
}
