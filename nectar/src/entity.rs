use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use uuid::Uuid;
use crate::logger;
use crate::scene;
use crate::archetype::{Archetype, ComponentID};
use crate::scene::ArchetypeSet;
pub type EntityID = Uuid;
pub type Type = HashSet<ComponentID>;

pub fn new_entity()-> EntityID { Uuid::new_v4() }

pub fn has_component(entity: EntityID, component: ComponentID, scene: &scene::Scene) -> Result<bool, Error>{
    dbg!(&scene.entity_index);
    dbg!(&entity);
    let at: &Archetype = match &scene.entity_index.get(&entity) {
        Some(x) => {x},
        None => return Err(Error::new(ErrorKind::NotFound, "Entity not Found!"))
    };
    let atset: &ArchetypeSet = &scene.component_index.get(&component).unwrap_or(return Err(Error::new(ErrorKind::NotFound, "Component not Found!")));
    Ok(atset.contains(&at.id))
}
pub fn add_component(entity: EntityID, component: ComponentID, scene: &mut scene::Scene){
    match scene.entity_index.get_mut(&entity) {
        Some(x) => {x.ty.insert(component);},
        None    => {logger::log("Entity does not exist in scene!", logger::LogType::ERROR);},
    }
}
