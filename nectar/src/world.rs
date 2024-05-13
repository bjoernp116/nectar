use std::any::Any;
use winit::event_loop::EventLoop;
use crate::eventloop;
use crate::events::Handler;
type Components = Vec<Box<dyn Component>>;
pub struct World {
    entities: Vec<Components>,
    entity_index: usize,
    pub event_handler: Handler,
}
impl World {
    pub fn new()->World{
        World {
            entities: vec![],
            entity_index: 0,
            event_handler: Handler::new()
        }
    }
    pub fn new_entity(&mut self)->usize{
        self.entities.push(vec![]);
        self.entity_index+=1;
        self.entity_index-1
    }
    pub fn add_component<C: Component + Copy + 'static>
        (&mut self, entity:usize, component: C) {
            if self.has_component::<C>(entity) {
                panic!("Entity already have component!");
            }
            self.entities[entity].push(Box::new(component));
    }

    pub fn get_components(&self, entity: usize)->Components{
        self.entities[entity]
            .iter()
            .map(|c| c.clone())
            .collect()
    }
    pub fn has_component<C: Component + Copy + 'static>
        (&self, entity: usize) -> bool {
            match self.get_component::<C>(entity) {
                Some(_) => true,
                None => false
            }
    }
    pub fn get_component<C: Component + Copy + 'static>
        (&self, entity: usize) -> Option<C> {
        for c in self.get_components(entity) {
            if let Some(&cmp) = c.as_any().downcast_ref::<C>() {
                return Some(cmp);
            }
        }
        None
    }
    pub fn start(&mut self){
        let rt = tokio::runtime::Runtime::new().unwrap();

        let event_loop = EventLoop::new().unwrap();

        match rt.block_on(eventloop::run(event_loop, &self)) {
            Err(e) => panic!("{}", e),
            _ => {},
        }

    }
}

pub trait Component{
    fn clone_box(&self) -> Box<dyn Component>;
    fn as_any(&self) -> &dyn Any;
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
#[macro_export]
macro_rules! component {
    ($t:ty) => {

        impl crate::world::Component for $t {
            fn clone_box(&self) -> Box<dyn crate::world::Component> {
                Box::new(self.clone())
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}
