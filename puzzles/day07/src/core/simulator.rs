use std::collections::hash_map::Entry;
use std::collections::HashMap;

use super::{Circuit, Input, Signal, Source};

type Memo<'a> = HashMap<&'a str, Signal>;

pub struct Simulator<'a> {
    circuit: &'a Circuit,
    memo: Memo<'a>,
}

impl<'a> Simulator<'a> {
    pub fn new(circuit: &'a Circuit) -> Self {
        Self {
            circuit,
            memo: Memo::new(),
        }
    }

    pub fn eval_wire(&mut self, wire: &'a str) -> Option<Signal> {
        if let Entry::Occupied(entry) = self.memo.entry(wire) {
            return Some(*entry.get());
        }

        let source = self.circuit.get(wire)?;
        let signal = self.eval_source(source)?;

        self.memo.insert(wire, signal);

        Some(signal)
    }

    pub fn override_wire(&mut self, wire: &'a str, signal: Signal) {
        self.memo.insert(wire, signal);
    }

    fn eval_input(&mut self, input: &'a Input) -> Option<Signal> {
        match input {
            Input::Constant(value) => Some(*value),
            Input::Wire(wire) => self.eval_wire(wire),
        }
    }

    fn eval_source(&mut self, source: &'a Source) -> Option<Signal> {
        match source {
            Source::Noop(input) => self.eval_input(input),
            Source::Not(input) => self.eval_input(input).map(|signal| !signal),
            Source::Or(input_a, input_b) => self
                .eval_input(input_a)
                .zip(self.eval_input(input_b))
                .map(|(signal_a, signal_b)| signal_a | signal_b),
            Source::And(input_a, input_b) => self
                .eval_input(input_a)
                .zip(self.eval_input(input_b))
                .map(|(signal_a, signal_b)| signal_a & signal_b),
            Source::ShiftLeft(input, dist) => self.eval_input(input).map(|signal| signal << dist),
            Source::ShiftRight(input, dist) => self.eval_input(input).map(|signal| signal >> dist),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn eval_wire() {
        let mut circuit = Circuit::new();

        macro_rules! connect {
            ($source:expr, $wire:expr) => {
                circuit.insert($wire.to_owned(), Source::try_from($source).unwrap());
            };
        }

        connect!("123", "x");
        connect!("456", "y");
        connect!("x AND y", "d");
        connect!("x OR y", "e");
        connect!("x LSHIFT 2", "f");
        connect!("y RSHIFT 2", "g");
        connect!("NOT x", "h");
        connect!("NOT y", "i");

        let mut simulator = Simulator::new(&circuit);

        macro_rules! test {
            ($wire:expr, $signal:expr) => {
                assert_eq!(simulator.eval_wire(&$wire).unwrap(), $signal);
            };
        }

        test!("d", 72);
        test!("e", 507);
        test!("f", 492);
        test!("g", 114);
        test!("h", 65412);
        test!("i", 65079);
        test!("x", 123);
        test!("y", 456);
    }
}
