use super::StateMachine;

pub struct ClothesMachine;


#[derive(PartialEq, Eq, Debug)]
// Piece of clothing through its lifecycle
pub enum ClothesState {
    Clean(u64),
    Dirty(u64),
    Wet(u64),
    Tattered,
}

#[allow(dead_code)]
pub enum ClothesAction {
    Wear,
    Wash,
    Dry,
}

impl StateMachine for ClothesMachine {
    type State = ClothesState;
    type Transition = ClothesAction;

    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            ClothesAction::Wear => {
                match starting_state {
                    ClothesState::Clean(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Dirty(i - 1)
                        }
                    }
                    ClothesState::Dirty(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Dirty(i - 1)
                        }
                    }
                    ClothesState::Wet(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Dirty(i - 1)
                        }
                    }
                    ClothesState::Tattered => ClothesState::Tattered
                }
            }
            ClothesAction::Wash => {
                match starting_state {
                    ClothesState::Clean(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Wet(i - 1)
                        }
                    }
                    ClothesState::Dirty(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Wet(i - 1)
                        }
                    }
                    ClothesState::Wet(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Wet(i - 1)
                        }
                    }
                    ClothesState::Tattered => ClothesState::Tattered
                }
            }
            ClothesAction::Dry => {
                match starting_state {
                    ClothesState::Clean(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Clean(i - 1)
                        }
                    }
                    ClothesState::Dirty(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Dirty(i - 1)
                        }
                    }
                    ClothesState::Wet(i) => {
                        if i - 1 == 0  || *i == 0 {
                            ClothesState::Tattered
                        } else {
                            ClothesState::Clean(i - 1)
                        }
                    }
                    ClothesState::Tattered => ClothesState::Tattered
                }
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn wear_clean_clothes() {
        let current_state = ClothesState::Clean(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Dirty(3);

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_dirty_clothes() {
        let current_state = ClothesState::Wet(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Dirty(3);

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_wet_clothes() {
        let current_state = ClothesState::Wet(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Dirty(3);

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_tattered_clothes() {
        let current_state = ClothesState::Tattered;
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Tattered;

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_clean_until_tattered() {
        let current_state = ClothesState::Clean(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Tattered;

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_wet_until_tattered() {
        let current_state = ClothesState::Wet(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Tattered;

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wear_dirty_until_tattered() {
        let current_state = ClothesState::Dirty(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wear);
        let expected_state = ClothesState::Tattered;

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wash_clean_clothes() {
        let current_state = ClothesState::Clean(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected_state = ClothesState::Wet(3);
        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn wash_dirty_clothes() {
    let current_state = ClothesState::Dirty(4);
    let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
    let expected = ClothesState::Wet(3);
    assert_eq!(new_state, expected);
}

    #[test]
    fn wash_wet_clothes() {
        let current_state = ClothesState::Wet(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected = ClothesState::Wet(3);
        assert_eq!(new_state, expected);
    }

    #[test]
    fn wash_tattered_clothes() {
        let current_state = ClothesState::Tattered;
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn wash_clean_until_tattered() {
        let current_state = ClothesState::Clean(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn wash_wet_until_tattered() {
        let current_state = ClothesState::Wet(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn wash_dirty_until_tattered() {
        let current_state = ClothesState::Dirty(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Wash);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_clean_clothes() {
        let current_state = ClothesState::Clean(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Clean(3);
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_dirty_clothes() {
        let current_state = ClothesState::Dirty(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Dirty(3);
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_wet_clothes() {
        let current_state = ClothesState::Wet(4);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Clean(3);
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_tattered_clothes() {
        let current_state = ClothesState::Tattered;
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_clean_until_tattered() {
        let current_state = ClothesState::Clean(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_cwet_until_tattered() {
        let current_state = ClothesState::Wet(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

    #[test]
    fn dry_dirty_until_tattered() {
        let current_state = ClothesState::Dirty(1);
        let new_state = ClothesMachine::next_state(&current_state, &ClothesAction::Dry);
        let expected = ClothesState::Tattered;
        assert_eq!(new_state, expected);
    }

}