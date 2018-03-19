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
pub trait Component: Any {
    /// init
    fn init(&mut self);

    /// Draw
    fn draw(&self);

    /// is_active
    fn is_active(&self) -> bool;

    /// destroy
    fn destroy(&mut self);

    /// get_name
    fn get_name(&self) -> &'static str;

    /// update
    fn update(&mut self);
}

/// Entity
pub struct Entity {
    entity_id: String,
    components: HashMap<TypeId, Box<Any>>,
}

impl Entity {
    /// Constructor
    pub fn new() -> Self {
        let entity_id = Uuid::new_v4().simple().to_string();

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
pub struct System {}
