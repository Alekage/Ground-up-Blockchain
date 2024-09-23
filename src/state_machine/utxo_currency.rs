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
                // clone state into memory from starting_state
                let mut new_state = starting_state.clone();
                let mut spends_amount = 0;
                let mut receives_amount = 0;
                let mut new_bills: Vec<Bill> = Vec::new();

                if spends.is_empty() {
                    return new_state
                }
                
                let spenders_len = spends.len();
                let last_serial = &spends[spenders_len - 1].serial;

                for spend in spends {
                    if new_state.bills.contains(spend) {
                        spends_amount += spend.amount;
                        if spend.serial != last_serial + 1 && !receives.is_empty() {
                            return new_state
                        }
                    }
                }

                for receive in receives {
                    receives_amount += receive.amount;

                    let bill = Bill {
                        owner: receive.owner,
                        amount: receive.amount,
                        serial: receive.serial,
                    };

                    new_bills.push(bill);
                }
                
                if spends_amount > receives_amount && !receives.is_empty() {
                    return new_state
                };

                for spend in spends {
                    if !new_state.bills.contains(spend) {
                        
                    }
                    new_state.bills.remove(spend);
                };

                for bill in new_bills {
                    new_state.add_bill(bill);
                }

                new_state
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

    #[test]
    fn sm_5_spending_and_receiving_same_bill_fails() {
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
                    amount: 20,
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

    #[test]
    fn sm_5_receiving_bill_with_incorrect_serial_fails() {
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
                receives: vec![
                    Bill {
                        owner: User::Alice,
                        amount: 10,
                        serial: u64::MAX,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 10,
                        serial: 4000,
                    },
                ],
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
    fn sm_5_spending_bill_with_incorrect_amount_fails() {
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
                    amount: 40,
                    serial: 0,
                }],
                receives: vec![Bill {
                    owner: User::Bob,
                    amount: 40,
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
    fn sm_5_spending_same_bill_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 40,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![
                    Bill {
                        owner: User::Alice,
                        amount: 40,
                        serial: 0,
                    },
                    Bill {
                        owner: User::Alice,
                        amount: 40,
                        serial: 0,
                    },
                ],
                receives: vec![
                    Bill {
                        owner: User::Bob,
                        amount: 20,
                        serial: 1,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 20,
                        serial: 2,
                    },
                    Bill {
                        owner: User::Alice,
                        amount: 40,
                        serial: 3,
                    },
                ],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 40,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_spending_more_than_bill_fails() {
        let start = State::from([
            Bill {
                owner: User::Alice,
                amount: 40,
                serial: 0,
            },
            Bill {
                owner: User::Charlie,
                amount: 42,
                serial: 1,
            },
        ]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![
                    Bill {
                        owner: User::Alice,
                        amount: 40,
                        serial: 0,
                    },
                    Bill {
                        owner: User::Charlie,
                        amount: 42,
                        serial: 1,
                    },
                ],
                receives: vec![
                    Bill {
                        owner: User::Bob,
                        amount: 20,
                        serial: 2,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 20,
                        serial: 3,
                    },
                    Bill {
                        owner: User::Alice,
                        amount: 52,
                        serial: 4,
                    },
                ],
            },
        );
        let expected = State::from([
            Bill {
                owner: User::Alice,
                amount: 40,
                serial: 0,
            },
            Bill {
                owner: User::Charlie,
                amount: 42,
                serial: 1,
            },
        ]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_spending_non_existent_bill_fails() {
        let start = State::from([Bill {
            owner: User::Alice,
            amount: 32,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Bob,
                    amount: 1000,
                    serial: 32,
                }],
                receives: vec![Bill {
                    owner: User::Bob,
                    amount: 1000,
                    serial: 33,
                }],
            },
        );
        let expected = State::from([Bill {
            owner: User::Alice,
            amount: 32,
            serial: 0,
        }]);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_spending_from_alice_to_all() {
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
                        amount: 10,
                        serial: 1,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 10,
                        serial: 2,
                    },
                    Bill {
                        owner: User::Charlie,
                        amount: 10,
                        serial: 3,
                    },
                ],
            },
        );
        let mut expected = State::from([
            Bill {
                owner: User::Alice,
                amount: 10,
                serial: 1,
            },
            Bill {
                owner: User::Bob,
                amount: 10,
                serial: 2,
            },
            Bill {
                owner: User::Charlie,
                amount: 10,
                serial: 3,
            },
        ]);
        expected.set_serial(4);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_spending_from_bob_to_all() {
        let start = State::from([Bill {
            owner: User::Bob,
            amount: 42,
            serial: 0,
        }]);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Bob,
                    amount: 42,
                    serial: 0,
                }],
                receives: vec![
                    Bill {
                        owner: User::Alice,
                        amount: 10,
                        serial: 1,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 10,
                        serial: 2,
                    },
                    Bill {
                        owner: User::Charlie,
                        amount: 22,
                        serial: 3,
                    },
                ],
            },
        );
        let mut expected = State::from([
            Bill {
                owner: User::Alice,
                amount: 10,
                serial: 1,
            },
            Bill {
                owner: User::Bob,
                amount: 10,
                serial: 2,
            },
            Bill {
                owner: User::Charlie,
                amount: 22,
                serial: 3,
            },
        ]);
        expected.set_serial(4);
        assert_eq!(end, expected);
    }

    #[test]
    fn sm_5_spending_from_charlie_to_all() {
        let mut start = State::from([
            Bill {
                owner: User::Charlie,
                amount: 68,
                serial: 54,
            },
            Bill {
                owner: User::Alice,
                amount: 4000,
                serial: 58,
            },
        ]);
        start.set_serial(59);
        let end = DigitalCashSystem::next_state(
            &start,
            &CashTransaction::Transfer {
                spends: vec![Bill {
                    owner: User::Charlie,
                    amount: 68,
                    serial: 54,
                }],
                receives: vec![
                    Bill {
                        owner: User::Alice,
                        amount: 42,
                        serial: 59,
                    },
                    Bill {
                        owner: User::Bob,
                        amount: 5,
                        serial: 60,
                    },
                    Bill {
                        owner: User::Charlie,
                        amount: 5,
                        serial: 61,
                    },
                ],
            },
        );
        let mut expected = State::from([
            Bill {
                owner: User::Alice,
                amount: 4000,
                serial: 58,
            },
            Bill {
                owner: User::Alice,
                amount: 42,
                serial: 59,
            },
            Bill {
                owner: User::Bob,
                amount: 5,
                serial: 60,
            },
            Bill {
                owner: User::Charlie,
                amount: 5,
                serial: 61,
            },
        ]);
        expected.set_serial(62);
        assert_eq!(end, expected);
    }
}