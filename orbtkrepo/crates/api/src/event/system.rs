use crate::event::Event;

pub enum SystemEvent {
    Quit,
}

impl Event for SystemEvent {}
