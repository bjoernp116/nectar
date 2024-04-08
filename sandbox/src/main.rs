use nectar::scene::Scene;
use nectar::entity::{self, new_entity};
use nectar::archetype::{new_component, Archetype, ComponentID};
fn main() {
    let mut world: Scene = Scene::new();
    let player: entity::EntityID = new_entity();
    let position: ComponentID = new_component();


    world.add_entity(player, Archetype::new());
    entity::add_component(player, position, &mut world);
    dbg!(&world.entity_index);
    dbg!(&world.archetype_index);
    dbg!(&world.component_index);
    assert_eq!(entity::has_component(player, position, &world).unwrap(), true);


}
