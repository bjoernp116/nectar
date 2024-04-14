use nectar::world::{World, Health};
fn main() {
    //nectar::eventloop::run();
    let mut w: World = World::new();
    let entity = w.new_entity();
    w.add_component::<Health>(entity, Health(10));
    w.add_component::<Health>(entity, Health(10));
    println!("{:?}", w.get_components(entity)[0]);
}
