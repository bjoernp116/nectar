pub enum EventType {
    Update,
    KeyInput(winit::event::KeyEvent),
    Custom(String),
}


pub struct Event {
    pub event_type: EventType,
    pub func: Box<dyn Fn()>,
}
