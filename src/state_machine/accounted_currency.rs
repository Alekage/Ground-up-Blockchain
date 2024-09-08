#![allow(dead_code)]

use super::{StateMachine, User};
use std::collections::HashMap;

pub struct AccountedCurrency {}

type Balances = HashMap<User, u64>;

pub enum AccountingTransaction {
    Mint {minter: User, amount: u64},
    Burn {burner: User, amount: u64},
    Transfer {sender: User, receiver: User, amount: u64}
}

impl StateMachine for AccountedCurrency {
    type State = Balances;
    type Transition = AccountingTransaction;

    fn next_state(_starting_state: &Self::State, _transition: &Self::Transition) -> Self::State {
        todo!()
    }
}