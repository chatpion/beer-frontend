pub use std::rc::Rc;

use orbtk::prelude::*;

use crate::data::{Angle, Direction};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UserEvent {
    Zero,
    Rotate(Angle, Angle),
    UpdatePos(Angle, Angle),
    BeginMove(Direction),
    EndMove(Direction)
}

pub type UserHandlerFn = dyn Fn(&mut StatesContext, &UserEvent) -> bool + 'static;

pub struct UserEventHandler {
    pub handler: Rc<UserHandlerFn>
}

impl Into<Rc<dyn EventHandler>> for UserEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for UserEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<UserEvent>() {
            return (self.handler)(states, event);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<UserEvent>()
    }
}


impl Event for UserEvent {}
