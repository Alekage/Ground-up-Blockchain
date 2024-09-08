mod swithces;
mod laundry;
mod atm;
mod accounted_currency;
pub trait StateMachine {
    type State;
    type Transition;


    fn next_state(starting_state: &Self::State, transition: &Self::Transition) -> Self::State;

    fn human_name() -> String {
        "Unnamed state machine".into()
    }

}
#[allow(dead_code)]
pub enum User {
    Alice,
    Bob,
    Charlie
}
