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
        match t {
            CashTransaction::Mint { minter, amount } => {
                let mut new_state = starting_state.clone();
                let bill: Bill = Bill { owner: *minter, amount: *amount, serial: 0 };
                new_state.add_bill(bill);
                new_state
            }
            CashTransaction::Transfer { spends, receives } => {
                State::new()
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn sm_5_mint_new_cash() {
        let start = State::new();
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Mint {
                minter: User::Alice,
                amount: 20,
            },
        );

        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_overflow_receives_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 42,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Alice,
                    amount: 42,
                    serial: 0,
                }],
                receives: vec![
                    Bill {
                        owner: User::Alice,
                        amount: u64::MAX,
                        serial: 1,
                    },
                    Bill {
                        owner: User::Alice,
                        amount: 42,
                        serial: 2,
                    },
                ],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 42,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_empty_spend_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![],
                receives: vec![Bill {
                    owner: User::Alice,
                    amount: 15,
                    serial: 1,
                }],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_empty_receive_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Alice,
                    amount: 20,
                    serial: 0,
                }],
                receives: vec![],
            },
        );
        let mut expected = State::from([]);
        expected.set_serial(1);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_output_value_0_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Alice,
                    amount: 20,
                    serial: 0,
                }],
                receives: vec![Bill {
                    owner: User::Bob,
                    amount: 0,
                    serial: 1,
                }],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_serial_number_already_seen_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Alice,
                    amount: 20,
                    serial: 0,
                }],
                receives: vec![Bill {
                    owner: User::Alice,
                    amount: 18,
                    serial: 0,
                }],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 20,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }
}