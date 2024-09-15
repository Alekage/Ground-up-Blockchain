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

#[cfg(test)]
pub mod tests {
    use super::*;


    #[test]
    fn simple_swipe_card() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };
        let end = Atm::next_state(&start, &Action::SwipeCard(1234));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn swipe_card_again_part_way_through() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: Vec::new(),
        };
        let end = Atm::next_state(&start, &Action::SwipeCard(1234));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);

        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: vec![Key::One, Key::Three],
        };
        let end = Atm::next_state(&start, &Action::SwipeCard(1234));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: vec![Key::One, Key::Three],
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn press_key_before_card_swipe() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::One));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn enter_single_digit_of_pin() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: Vec::new(),
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::One));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: vec![Key::One],
        };

        assert_eq!(end, expected);

        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: vec![Key::One],
        };
        let end1 = Atm::next_state(&start, &Action::PressKey(Key::Two));
        let expected1 = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(1234),
            keystroke_register: vec![Key::One, Key::Two],
        };

        assert_eq!(end1, expected1);
    }

    #[test]
    fn enter_wrong_pin() {
        // Create hash of pin
        let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
        let pin_hash = crate::hash(&pin);

        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(pin_hash),
            keystroke_register: vec![Key::Three, Key::Three, Key::Three, Key::Three],
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn enter_correct_pin() {
        // Create hash of pin
        let pin = vec![Key::One, Key::Two, Key::Three, Key::Four];
        let pin_hash = crate::hash(&pin);

        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticating(pin_hash),
            keystroke_register: vec![Key::One, Key::Two, Key::Three, Key::Four],
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn enter_single_digit_of_withdraw_amount() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: Vec::new(),
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::One));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: vec![Key::One],
        };

        assert_eq!(end, expected);

        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: vec![Key::One],
        };
        let end1 = Atm::next_state(&start, &Action::PressKey(Key::Four));
        let expected1 = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: vec![Key::One, Key::Four],
        };

        assert_eq!(end1, expected1);
    }

    #[test]
    fn try_to_withdraw_too_much() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: vec![Key::One, Key::Four],
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
        let expected = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

    #[test]
    fn withdraw_acceptable_amount() {
        let start = Atm {
            cash_inside: 10,
            expected_pin_hash: Auth::Authenticated,
            keystroke_register: vec![Key::One],
        };
        let end = Atm::next_state(&start, &Action::PressKey(Key::Enter));
        let expected = Atm {
            cash_inside: 9,
            expected_pin_hash: Auth::Waiting,
            keystroke_register: Vec::new(),
        };

        assert_eq!(end, expected);
    }

}