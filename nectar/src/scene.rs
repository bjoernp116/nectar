use std::collections::{HashMap, HashSet};
use crate::{archetype::{self, Archetype, ArchetypeID, ComponentID}, entity};
pub type ArchetypeSet = HashSet<ArchetypeID>;

pub struct Scene{
    pub entity_index: HashMap<entity::EntityID, Archetype>,
    pub archetype_index: HashMap<entity::Type, Archetype>,
    pub component_index: HashMap<ComponentID, ArchetypeSet>,
}
impl Scene{
    pub fn new()->Scene{
        Scene{
            entity_index: HashMap::new(),
            archetype_index: HashMap::new(),
            component_index: HashMap::new()
        }
    }
    pub fn add_entity(&mut self, entity: entity::EntityID, archetype: archetype::Archetype){
        self.entity_index.insert(entity, archetype);
    }
}
