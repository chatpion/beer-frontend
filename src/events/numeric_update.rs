pub use std::rc::Rc;

use orbtk::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum NumericUpdateEvent {
    Update(isize),
}

pub type NumericUpdateHandlerFn = dyn Fn(&mut StatesContext, &NumericUpdateEvent) -> bool + 'static;

pub struct NumericUpdateEventHandler {
    pub handler: Rc<NumericUpdateHandlerFn>
}

impl Into<Rc<dyn EventHandler>> for NumericUpdateEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for NumericUpdateEventHandler {
    fn handle_event(&self, states: &mut StatesContext, event: &EventBox) -> bool {
        if let Ok(event) = event.downcast_ref::<NumericUpdateEvent>() {
            return (self.handler)(states, event);
        }

        false
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<NumericUpdateEvent>()
    }
}


impl Event for NumericUpdateEvent {}
