use nectar::world::World;
use nectar::defaults::Transform3D;
use nectar::events::*;
fn main() {

    let mut w: World = World::new();
    let entity = w.new_entity();
    w.add_component::<Transform3D>(entity, Transform3D::new());
    //w.add_component::<Renderer>(entity, Renderer::new());
    let mut _transform = w.get_component::<Transform3D>(entity).unwrap();
    fn p(){
        print!("Hello World");
    }
    w.event_handler.add_event(EventType::KeyInput(KeyCode::Escape), p);
    w.start();
}
