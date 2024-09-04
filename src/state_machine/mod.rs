mod swithces;
mod laundry;
mod atm;
pub trait StateMachine {
    type State;
    type Transition;


    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State;

    fn human_name() -> String {
        "Unnamed state machine".into()
    }

}
