use super::StateMachine;

pub struct LightSwitch;


impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    fn next_state(starting_state: &bool, _transition: &()) -> Self::State {
        match starting_state {
            true => false,
            false => true,
        }
    }
}

struct WeirdStateMachine;


#[derive(PartialEq, Eq, Debug)]
pub struct TwoSwitches {
    first_switch: bool,
    second_switch: bool
}

#[allow(dead_code)]
impl TwoSwitches {
    fn new(first_switch: bool, second_switch: bool) -> Self {
        TwoSwitches {
            first_switch,
            second_switch
        }
    }
}

#[allow(dead_code)]
enum Toggle {
    FirstSwitch,
    SecondSwitch
}

impl StateMachine for WeirdStateMachine {
    type State = TwoSwitches;
    type Transition = Toggle;

    fn next_state(starting_state: &TwoSwitches, transition: &Toggle) -> Self::State {
        match transition {
            Toggle::FirstSwitch => {
                if starting_state.first_switch == true {
                    TwoSwitches {
                        first_switch: false,
                        second_switch: false
                    }
                } else {
                    TwoSwitches {
                        first_switch: true,
                        second_switch: starting_state.second_switch
                    }
                }
            }

            Toggle::SecondSwitch => {
                if starting_state.second_switch == true {
                    TwoSwitches {
                        first_switch: starting_state.first_switch,
                        second_switch: false
                    } 
                } else {
                    TwoSwitches {
                        first_switch: starting_state.first_switch,
                        second_switch: true
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
    fn light_switch_toggles_off() {
        let current_state = &true;
        assert!(!LightSwitch::next_state(current_state, &()))
    }

    #[test]
    fn light_switch_toggles_on() {
        let current_state = &false;
        assert!(LightSwitch::next_state(current_state, &()))
    }

    #[test]
    fn two_switches_first_on() {
        let current_state = TwoSwitches::new(false, false);
        let next_state = TwoSwitches::new(true, false);

        assert_eq!(WeirdStateMachine::next_state(&current_state, &Toggle::FirstSwitch), next_state);
    }

    #[test]
    fn two_switches_first_off_second_was_on() {
        let current_state = TwoSwitches::new(true, true);
        let next_state = TwoSwitches::new(false, false);

        assert_eq!(WeirdStateMachine::next_state(&current_state, &Toggle::FirstSwitch), next_state);
    }

    #[test]
    fn two_switches_first_off_second_was_off() {
        let current_state = TwoSwitches::new(true, false);
        let next_state = TwoSwitches::new(false, false);

        assert_eq!(WeirdStateMachine::next_state(&current_state, &Toggle::FirstSwitch), next_state);
    }

     #[test]
     fn two_switches_second_goes_on() {
        let current_state = TwoSwitches::new(false, false);
        let next_state = TwoSwitches::new(false, true);

        assert_eq!(WeirdStateMachine::next_state(&current_state, &Toggle::SecondSwitch), next_state);
     }

     #[test]
     fn two_switches_second_goes_off() {
        let current_state = TwoSwitches::new(true, false);
        let next_state = TwoSwitches::new(true, true);

        assert_eq!(WeirdStateMachine::next_state(&current_state, &Toggle::SecondSwitch), next_state);
     }
}