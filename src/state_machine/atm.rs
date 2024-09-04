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
    expected_pin_cash: Auth,
    keystroke_register: Vec<Key>
}

impl StateMachine for Atm {
    type State = Self;
    type Transition = Action;

    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            Action::SwipeCard(i) => {
                match starting_state.expected_pin_cash {
                    Auth::Waiting => return starting_state.clone(),
                    Auth::Authenticating(i) => return starting_state.clone(),
                    Auth::Authenticated => return starting_state.clone()
                }
            }
            Action::PressKey(i) => return starting_state.clone()
        }
    }
}