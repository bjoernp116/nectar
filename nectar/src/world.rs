
#[derive(Debug, Clone)]
pub struct Health(pub i32);

type Components = Vec<Box<dyn Component>>;
pub struct World {
    entities: Vec<Components>,
    entity_index: usize,
}
impl World {
    pub fn new()->World{
        World {
            entities: vec![],
            entity_index: 0,
        }
    }
    pub fn new_entity(&mut self)->usize{
        self.entities.push(vec![]);
        self.entity_index+=1;
        self.entity_index-1
    }
    pub fn add_component<T: Component + 'static>(&mut self, entity:usize, component: T){
        self.entities[entity].push(Box::new(component));
    }
    pub fn get_components(&self, entity: usize)->Components{
        self.entities[entity]
            .iter()
            .map(|c| c.clone())
            .collect()
    }

}
pub trait Component {
    fn clone_box(&self) -> Box<dyn Component>;
}
impl Component for Health{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}
impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Component")
    }
}

