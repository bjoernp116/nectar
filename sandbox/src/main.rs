use nectar::scene::Scene;
use nectar::entity::{self, new_entity};
use nectar::archetype::{new_component, Archetype, ComponentID};
fn main() {
    let mut world: Scene = Scene::new();
    nectar::eventloop::run();
}
