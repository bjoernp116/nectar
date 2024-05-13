use nectar::world::World;
use nectar::defaults::Transform3D;
fn main() {

    let mut w: World = World::new();
    let entity = w.new_entity();
    w.add_component::<Transform3D>(entity, Transform3D::new());
    //w.add_component::<Renderer>(entity, Renderer::new());
    let mut _transform = w.get_component::<Transform3D>(entity).unwrap();
    w.start();
}
