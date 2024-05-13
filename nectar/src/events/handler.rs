use crate::event::*;

pub struct Handler {
    events: Vec<Event>
}

impl Handler {
    fn add_event<F>(&mut self, et: EventType, func: F)
    where F: Fn(){
        events.push(
            Event {
                event_type: et,
                func: func,
            }
        );
    }
    fn call_events(&self, et: EventType){
        for e in self.events {
            if matches!(e.event_type, et) {
                e.func();
            }
        }
    }
}
