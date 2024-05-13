use crate::events::*;

pub struct Handler {
    events: Vec<Event>
}

impl Handler {
    pub fn new() -> Handler {
        Handler { events: vec![] }
    }
    pub fn add_event<F: 'static>(&mut self, et: EventType, func: F)
    where F: Fn(){
        self.events.push(
            Event {
                event_type: et,
                func: Box::new(func),
            }
        );
    }
    pub fn call_events(&self, et: EventType){
        for e in &self.events {
            if matches!(&e.event_type, et) {
                (e.func)();
            }
        }
    }
}
