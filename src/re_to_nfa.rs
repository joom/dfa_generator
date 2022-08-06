use crate::regex::Regex;
use crate::regex::Regex::*;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum State {
    Start,
    Standard,
    Final,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Start => f.write_fmt(format_args!("Start")),
            State::Standard => f.write_fmt(format_args!("")),
            State::Final => f.write_fmt(format_args!("Final")),
        }
    }
}

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
pub enum BranchLabel<T> {
    Literal(T),
    Empty,
}

impl<T: fmt::Debug> fmt::Debug for BranchLabel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchLabel::Literal(c) => f.write_fmt(format_args!("{:?}", c)),
            BranchLabel::Empty => f.write_fmt(format_args!("ε")),
        }
    }
}

pub struct NFA<T> {
    pub graph: Graph<State, BranchLabel<T>>,
}

impl<T: Clone> Regex<T> {
    pub fn to_nfa(&self) -> NFA<T> {
        NFA {
            graph: converter(self.clone()),
        }
    }
}

pub fn converter<T>(expression: Regex<T>) -> Graph<State, BranchLabel<T>> {
    let mut graph = Graph::<State, BranchLabel<T>>::new();
    let start = graph.add_node(State::Start);
    let end = generate(&mut graph, start, expression);
    *(graph.node_weight_mut(end).unwrap()) = State::Final;
    return graph;
}

fn generate<T>(
    graph: &mut Graph<State, BranchLabel<T>>,
    start: NodeIndex<u32>,
    expression: Regex<T>,
) -> NodeIndex<u32> {
    match expression {
        Literal(s) => symbol(graph, start, s),
        Concatenation(a, b) => concatenation(graph, start, *a, *b),
        Alternative(a, b) => alternative(graph, start, *a, *b),
        Star(a) => kleene_star(graph, start, *a),
        Empty => empty(graph, start),
    }
}

fn concatenation<T>(
    graph: &mut Graph<State, BranchLabel<T>>,
    start: NodeIndex<u32>,
    a: Regex<T>,
    b: Regex<T>,
) -> NodeIndex<u32> {
    let mid = generate(graph, start, a);

    let end = generate(graph, mid, b);
    return end;
}

fn alternative<T>(
    graph: &mut Graph<State, BranchLabel<T>>,
    start: NodeIndex<u32>,
    a: Regex<T>,
    b: Regex<T>,
) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let end_a = generate(graph, start, a);

    let edge = graph.add_edge(end_a, end, BranchLabel::Empty);
    let end_b = generate(graph, start, b);
    let edge = graph.add_edge(end_b, end, BranchLabel::Empty);
    return end;
}

fn kleene_star<T>(
    graph: &mut Graph<State, BranchLabel<T>>,
    start: NodeIndex<u32>,
    a: Regex<T>,
) -> NodeIndex<u32> {
    let node = graph.add_node(State::Standard);
    let end = generate(graph, node, a);
    let edge = graph.add_edge(end, node, BranchLabel::Empty);
    let edge = graph.add_edge(start, node, BranchLabel::Empty);
    return end;
}

fn symbol<T>(
    graph: &mut Graph<State, BranchLabel<T>>,
    start: NodeIndex<u32>,
    s: T,
) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Literal(s));
    return end;
}

fn empty<T>(graph: &mut Graph<State, BranchLabel<T>>, start: NodeIndex<u32>) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Empty);
    return end;
}
