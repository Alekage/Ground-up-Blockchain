#![allow(dead_code)]
use super::StateMachine;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
pub enum Key {
    One, 
    Two,
    Three, 
    Four,
    Enter
}

pub enum Action {
    SwipeCard(u64),
    PressKey(Key)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Auth {
    Waiting, 
    Authenticating(u64),
    Authenticated
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Atm {
    cash_inside: u64,
    expected_pin_hash: Auth,
    keystroke_register: Vec<Key>
}

impl StateMachine for Atm {
    type State = Self;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, t: &Self::Transition) -> Self::State {
        match t {
            Action::SwipeCard(i) => {
                match starting_state.expected_pin_hash {
                    Auth::Waiting => {
                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticating(*i),
                            keystroke_register: starting_state.keystroke_register.clone()

                        }
                    }
                    Auth::Authenticating(pin_hash) => {
                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticating(pin_hash),
                            keystroke_register: starting_state.keystroke_register.clone()
                        }
                    }
                    Auth::Authenticated => {
                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticated,
                            keystroke_register: starting_state.keystroke_register.clone()
                        }
                    }
                }
            }
            Action::PressKey(i) => {
                match starting_state.expected_pin_hash {
                    Auth::Waiting => {
                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Waiting,
                            keystroke_register: starting_state.keystroke_register.clone()

                        }
                    }
                    Auth::Authenticating(pin_hash) => {
                        let mut original = starting_state.keystroke_register.clone();
                        original.push(i.clone());

                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticating(pin_hash),
                            keystroke_register: original
                        }
                    }
                    Auth::Authenticated => {
                        let mut original = starting_state.keystroke_register.clone();
                        original.push(i.clone());
                        
                        return Self {
                            cash_inside: starting_state.cash_inside,
                            expected_pin_hash: Auth::Authenticated,
                            keystroke_register: original
                        }
                    }
                }
            }
        }
    }
}