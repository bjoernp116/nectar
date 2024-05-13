pub enum EventType {
    Update,
    KeyInput(winit::event::KeyEvent),
    Custom(String),
}


pub struct Event {
    event_type: EventType,
    func: Box<dyn Fn()>,
}
