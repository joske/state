use statig::{prelude::*, StateOrSuperstate};

#[derive(Default)]
struct MyState {}

#[derive(Debug)]
pub enum Event {
    OPEN,
    OPENED,
    CLOSE,
    CLOSED,
}

#[state_machine(
    initial = "State::closed()",
    on_dispatch = "Self::on_dispatch",
    on_transition = "Self::on_transition",
    state(derive(Debug)),
    superstate(derive(Debug))
)]
impl MyState {
    #[state]
    fn closed(event: &Event) -> Response<State> {
        match event {
            Event::OPEN => Transition(State::opening()),
            _ => Super,
        }
    }

    #[state]
    fn opening(event: &Event) -> Response<State> {
        match event {
            Event::OPENED => Transition(State::open()),
            _ => Super,
        }
    }

    #[state]
    fn open(event: &Event) -> Response<State> {
        match event {
            Event::CLOSE => Transition(State::closing()),
            _ => Super,
        }
    }

    #[state]
    fn closing(event: &Event) -> Response<State> {
        match event {
            Event::CLOSED => Transition(State::closed()),
            _ => Super,
        }
    }
}

impl MyState {
    fn on_transition(&mut self, source: &State, target: &State) {
        println!("transitioned from `{:?}` to `{:?}`", source, target);
    }

    fn on_dispatch(&mut self, state: StateOrSuperstate<MyState>, event: &Event) {
        println!("dispatched `{:?}` to `{:?}`", event, state);
    }
}

fn main() {
    let machine = MyState::default().state_machine();
    let mut machine = machine.init();
    machine.handle(&Event::OPEN);
    machine.handle(&Event::OPEN);
    machine.handle(&Event::OPENED);
    machine.handle(&Event::CLOSE);
    machine.handle(&Event::CLOSED);
}
