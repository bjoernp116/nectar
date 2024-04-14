use std::cell::{RefCell, RefMut};
#[derive(PartialEq, Debug)]
pub struct Health(pub i32);
#[derive(PartialEq, Debug)]
pub struct Name(pub &'static str);
#[derive(PartialEq, Debug)]
pub struct Money(pub i32);

pub struct World {
    entities: usize,
    components: Vec<Box<dyn Component>>

}

impl World {
    pub fn new()->Self{
        Self {
            entities: 0,
            components: Vec::new()
        }
    }
    pub fn new_entity(&mut self) -> usize{
        let entity_id = self.entities;
        for components in self.components.iter_mut() {
            components.push_none();
        }
        self.entities += 1;
        entity_id
    }
    pub fn add_component<ComponentType: 'static>
        (&mut self, entity: usize, component: ComponentType){
        for components in self.components.iter_mut() {
            if let Some(components) = components.as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>(){
                    components.get_mut()[entity] = Some(component);
                    return;
            }
        }
        let mut new_components: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities);

        for _ in 0..self.entities {
            new_components.push(None);
        }
        new_components[entity] = Some(component);
        self.components.push(Box::new(RefCell::new(new_components)));

    }

    pub fn borrow_components_mut<ComponentType: 'static>(&self)
        -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for components in self.components.iter() {
            if let Some(components) = components
                .as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>(){
                return Some(components.borrow_mut());
            }
        }
        None
    }

    pub fn has_component<T: std::cmp::PartialEq + 'static>(&self, entity: usize)->bool{
        self.borrow_components_mut::<T>().unwrap()[entity] != None
    }
}
pub trait Component {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}


impl<T: 'static> Component for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}
