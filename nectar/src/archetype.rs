use std::collections::HashSet;
use uuid::Uuid;
pub type ComponentID = Uuid;
pub fn new_component()-> ComponentID { Uuid::new_v4() }
pub type ArchetypeID = Uuid;
pub type Column<T> = Vec<T>;
#[derive(Debug)]
pub struct Archetype {
    pub id: ArchetypeID,
    pub ty: HashSet<ComponentID>,
    pub components: Vec<Column<ComponentID>>
}
impl Archetype {
    pub fn new()->Archetype{
        Archetype {
            ty: HashSet::new(),
            id: Uuid::new_v4(),
            components: vec![]
        }
    }
    pub fn from(components: Vec<ComponentID>)->Archetype{
        Archetype{
            ty: HashSet::from_iter(components),
            id: Uuid::new_v4(),
            components: vec![]
        }
    }
}
