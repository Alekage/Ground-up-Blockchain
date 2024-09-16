#![allow(dead_code, unused_variables)]

use super::{StateMachine, User};
use std::collections::HashSet;

pub struct DigitalCashSystem;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Bill {
    owner: User,
    amount: u64,
    serial: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    bills: HashSet<Bill>,
    next_serial: u64,
}

impl State {
    pub fn new() -> Self {
        State {
            bills: HashSet::<Bill>::new(),
            next_serial: 0,
        }
    }

    pub fn set_serial(&mut self, serial: u64) {
        self.next_serial = serial;
    }

    pub fn next_serial(&self) -> u64 {
        self.next_serial
    }

    fn increment_serial(&mut self) {
        self.next_serial += 1
    }

    fn add_bill(&mut self, elem: Bill) {
        self.bills.insert(elem);
        self.increment_serial()
    }
}

impl FromIterator<Bill> for State {
    fn from_iter<I: IntoIterator<Item = Bill>>(iter: I) -> Self {
        let mut state = State::new();

        for i in iter {
            state.add_bill(i)
        }
        state
    }
}

impl<const N: usize> From<[Bill; N]> for State {
    fn from(value: [Bill; N]) -> Self {
        State::from_iter(value)
    }
}


pub enum CashTransaction {
    Mint { minter: User, amount: u64 },
    Transfer {
        spends: Vec<Bill>,
        receives: Vec<Bill>,
    },
}

impl StateMachine for DigitalCashSystem {
    type State = State;
    type Transition = CashTransaction;

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        todo!()
    }
}
