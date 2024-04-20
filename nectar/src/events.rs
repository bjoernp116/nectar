#![allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum EventType {
    START,
    UPDATE,
    KEYPRESSED,
    KEYRELEASED,
    MOUSEBUTTONPRESSED,
    MOUSEBUTTENRELEASED,
    MOUSEMOVED,
    MOUSESCROLLED,
}
struct UpdateEvent {
    evnet_type: EventType,

}

/*pub trait System {}
pub trait Handler<S: System, E: Event> {
    fn handle(&self, system: &mut S, event: &E);
}
impl<S: System, E: Event, F: Fn(&mut S, &E)> Handler<S, E> for F {
    fn handle(&self, system: &mut S, event: &E) {
        self(system, event);
    }
}
*/
