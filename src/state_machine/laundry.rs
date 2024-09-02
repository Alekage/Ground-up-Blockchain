use super::StateMachine;

pub struct ClothesMachine;

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