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
    fn mint_creates_account() {
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
    fn mint_creates_second_account() {
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
    fn mint_increases_balance() {
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
    fn empty_mint() {
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
    fn simple_burn() {
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
    fn burn_no_existential_deposit_left() {
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
    fn non_registered_burner() {
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
    fn burn_more_than_balance() {
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

    #[test]
    fn empty_burn() {
        let start = HashMap::from([(User::Alice, 100)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Burn {
                burner: User::Alice,
                amount: 0,
            },
        );
        let expected = HashMap::from([(User::Alice, 100)]);

        assert_eq!(end, expected);
}

    #[test]
    fn burner_does_not_exist() {
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
    fn simple_transfer() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Alice,
                receiver: User::Bob,
                amount: 10,
            },
        );
        let expected = HashMap::from([(User::Alice, 90), (User::Bob, 60)]);

        assert_eq!(end, expected);

        let start = HashMap::from([(User::Alice, 90), (User::Bob, 60)]);
        let end1 = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Bob,
                receiver: User::Alice,
                amount: 50,
            },
        );
        let expected1 = HashMap::from([(User::Alice, 140), (User::Bob, 10)]);

        assert_eq!(end1, expected1);
    }

    #[test]
    fn send_to_same_user() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Bob,
                receiver: User::Bob,
                amount: 10,
            },
        );
        let expected = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn insufficient_balance_transfer() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Bob,
                receiver: User::Alice,
                amount: 60,
            },
        );
        let expected = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn sender_not_registered() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Charlie,
                receiver: User::Alice,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn receiver_not_registered() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Alice,
                receiver: User::Charlie,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 50), (User::Bob, 50), (User::Charlie, 50)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn sender_to_empty_balance() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Bob,
                receiver: User::Alice,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 150)]);

        assert_eq!(end, expected);
    }

    #[test]
    fn transfer() {
        let start = HashMap::from([(User::Alice, 100), (User::Bob, 50)]);
        let end = AccountedCurrency::next_state(
            &start,
            &AccountingTransaction::Transfer {
                sender: User::Bob,
                receiver: User::Charlie,
                amount: 50,
            },
        );
        let expected = HashMap::from([(User::Alice, 100), (User::Charlie, 50)]);

        assert_eq!(end, expected);
    }
}