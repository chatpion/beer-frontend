pub use std::rc::Rc;

use orbtk::prelude::*;

use crate::data::Angle;

#[derive(Copy, Clone, PartialEq)]
pub enum AngleEvent {
    UpdateRightAsc(Angle),
    UpdateDeclination(Angle)
}

pub type AngleHandlerFn = dyn Fn(&mut StatesContext, &AngleEvent) -> bool + 'static;

pub struct AngleEventHandler {
    pub handler: Rc<AngleHandlerFn>
}

impl Into<Rc<dyn EventHandler>> for AngleEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for AngleEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<AngleEvent>() {
            return (self.handler)(states, event);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<AngleEvent>()
    }
}


impl Event for AngleEvent {}
