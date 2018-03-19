//! Simple Entity Component System
//!
//! Convert javascript original [How to Build an Entity Component System Game in Javascript](http://vasir.net/blog/game-development/how-to-build-entity-component-system-in-javascript)
//! Shamelessly copied from [Rustic Entity-Component System](https://github.com/AndyBarron/rustic-ecs)

extern crate uuid;

use uuid::Uuid;
use std::collections::HashMap;
use std::any::{Any, TypeId};

/// NotFound
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NotFound {
    /// Entity
    Entity,
    /// Component
    Component,
}

/// ECS Result
pub type EcsResult<T> = Result<T, NotFound>;

/// Component trait
pub trait Component: Any {}

impl<T: Any> Component for T {}
/// Entity
pub struct Entity {
    entity_id: String,
    components: HashMap<TypeId, Box<Any>>,
}

impl Entity {
    /// Constructor
    pub fn new(entity_id: String) -> Self {
        Entity {
            entity_id: entity_id,
            components: HashMap::new(),
        }
    }

    /// add_component
    pub fn add_component<C: Component>(&mut self, c: C) {
        self.components.insert(TypeId::of::<C>(), Box::new(c));
    }

    /// get_compoponent
    pub fn get_component<C: Component>(&self) -> EcsResult<&C> {
        self.components
            .get(&TypeId::of::<C>())
            .map(|c| {
                c.downcast_ref()
                    .expect("Component borrow: Internal downcast error")
            })
            .ok_or_else(|| NotFound::Component)
    }

    /// get_component_mut
    pub fn get_component_mut<C: Component>(&mut self) -> EcsResult<&mut C> {
        match self.components.get_mut(&TypeId::of::<C>()) {
            Some(c) => Ok(c.downcast_mut()
                .expect("Compoment borrow map: Internal downcast error")),
            None => Err(NotFound::Component),
        }
    }

    /// destroy component
    pub fn destory_component<C: Component>(&mut self) {
        self.components.remove(&TypeId::of::<C>()).unwrap();
    }
}

/// System
#[derive(Default)]
pub struct System {
    /// Entry Map
    pub entities: HashMap<String, Entity>,
}

impl System {
    /// new
    pub fn new() -> Self {
        Default::default()
    }

    /// create_entity
    pub fn create_entity(&mut self) -> &String {
        let entity_id = Uuid::new_v4().simple().to_string();

        self.entities
            .insert(entity_id.clone(), Entity::new(entity_id.clone()));
        &self.entities.get(&entity_id).unwrap().entity_id
    }

    /// set method
    pub fn set<C: Component>(&mut self, id: String, comp: C) {
        match self.entities.get_mut(&id) {
            Some(ele) => ele.add_component::<C>(comp),
            _ => {}
        };
    }

    /// get method
    pub fn get<C: Component>(&self, id: String) -> EcsResult<&C> {
        match self.entities.get(&id) {
            Some(ele) => ele.get_component::<C>(),
            None => Err(NotFound::Entity),
        }
    }

    /// mutable get method
    pub fn get_mut<C: Component>(&mut self, id: String) -> EcsResult<&mut C> {
        match self.entities.get_mut(&id) {
            Some(ele) => ele.get_component_mut::<C>(),
            None => Err(NotFound::Entity),
        }
    }
}
