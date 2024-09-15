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

    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State {
        let mut new_state = starting_state.clone();

        match transition {
            AccountingTransaction::Mint { minter, amount } => {
                if *amount > 0 {
                    let balance = new_state.entry(*minter).or_insert(0);
                    *balance += amount;
                } else {
                    if let Some(balance) = new_state.get(minter) {
                        if *balance == 0 {
                            new_state.remove(minter);
                        }
                    }
                }
            },

            AccountingTransaction::Burn { burner, amount } => {
                if let Some(balance) = new_state.get_mut(burner) {
                    if *balance > *amount {
                        *balance -= amount;
                    } else {
                        new_state.remove(burner);
                    }
                }
            },
            AccountingTransaction::Transfer { sender, receiver, amount } => {
                if let Some(sender_balance) = new_state.get_mut(sender) {
                    if *sender_balance >= *amount {
                        *sender_balance -= amount;
                        let receiver_balance = new_state.entry(*receiver).or_insert(0);
                        *receiver_balance += amount;
                    }
                }
            }   
        }

        return new_state
    }
}


#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn sm_4_mint_creates_account() {
        let start = HashMap::new();
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Mint {
                minter: User::Alice,
                amount: 100,
            },
        );
        let expected = HashMap::from([(User::Alice, 100)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn sm_4_mint_creates_second_account() {
        let start = HashMap::from([(User::Alice, 100)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Mint {
                minter: User::Bob,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn sm_4_mint_increases_balance() {
        let start = HashMap::from([(User::Alice, 100)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Mint {
                minter: User::Alice,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 150)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn sm_4_empty_mint() {
        let start = HashMap::new();
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Mint {
                minter: User::Alice,
                amount: 0,
            },
        );
        let expected = HashMap::new();

        assert_eq!(end, expected);
    }

    #[test]
    fn sm_4_simple_burn() {
        let start = HashMap::from([(User::Alice, 100)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Burn {
                burner: User::Alice,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 50)]);
    
        assert_eq!(end, expected);
    }
    
    #[test]
    fn sm_4_burn_no_existential_deposit_left() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Burn {
                burner: User::Bob,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 100)]);
    
        assert_eq!(end, expected);
    }
    
    #[test]
    fn sm_4_non_registered_burner() {
        let start = HashMap::from([(User::Alice, 100)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Burn {
                burner: User::Bob,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 100)]);
    
        assert_eq!(end, expected);
    }
    
    #[test]
    fn sm_4_burn_more_than_balance() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end2 = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Burn {
                burner: User::Bob,
                amount: 100,
            },
        );
        let expected2 = HashMap::from([(User::Alice, 100)]);
    
        assert_eq!(end2, expected2);
    }
}