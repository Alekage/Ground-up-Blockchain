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

pub fn key_to_digit(keys: &Vec<Key>) -> u64 {
    let mut number: u64 = 0;

    for key in keys {
        number = match key {
            Key::Enter => number, 
            _ => number * 10 + match key {
                Key::One => 1,
                Key::Two => 2,
                Key::Three => 3,
                Key::Four => 4,
                _ => 0,
            },
        };
    }

    number
}

impl StateMachine for Atm {
    // Notice that we are using the same type for the state as we are using for the machine this time.
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
                        
                        if *i != Key::Enter {
                            original.push(i.clone());
                        }

                        if *i == Key::Enter && pin_hash != crate::hash(&original) {
                            return Self {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new()
                            }
                        } else if pin_hash == 1234 {
                            return Self {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Authenticating(1234),
                                keystroke_register: original
                            }
                        } else {
                            return Self {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Authenticated,
                                keystroke_register: Vec::new()
                            }
                        }
                        
                    }
                    Auth::Authenticated => {
                        let mut original = starting_state.keystroke_register.clone();
                        
                        if *i != Key::Enter {
                            original.push(i.clone());
                            return Self {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Authenticated,
                                keystroke_register: original
                            }
                        }


                        if key_to_digit(&original) > starting_state.cash_inside {
                            return Self {
                                cash_inside: starting_state.cash_inside,
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new()
                            }
                        } else {
                            return Self {
                                cash_inside: starting_state.cash_inside - key_to_digit(&original),
                                expected_pin_hash: Auth::Waiting,
                                keystroke_register: Vec::new()
                            }
                        }

                    }
                }
            }
        }
    }
}