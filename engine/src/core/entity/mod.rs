extern crate cgmath;

pub mod cuboid;
pub mod component;

use self::cgmath::{
    VectorSpace, Rotation,
    Point3, Vector3, Quaternion
};

use core::{Event, EventEmitter};

use self::component::Component;

use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// TODO: implement Renderable

/// Holds common virtual world object's properties and components.
pub struct Entity {
    prop_container: PropContainer,
    components: Vec<Rc<Any>>,
    emitter: EventEmitter<Any, PropContainer>
}

impl Entity {
    /// Create a new entity.
    pub fn new() -> Entity {
        return Entity {
            prop_container: PropContainer::new(),
            components: Vec::new(),
            emitter: EventEmitter::new()
        };
    }

    /// Create a new entity from properties.
    pub fn from(position: Point3<f32>, orientation: Quaternion<f32>, scale: f32) -> Entity {
        return Entity {
            prop_container: PropContainer::from(position, orientation, scale),
            components: Vec::new(),
            emitter: EventEmitter::new()
        };
    }

    /// Translate the entity `n` units towards it's orientation direction.
    ///
    /// Negative value indicates backwards translation.
    #[inline]
    pub fn move_by(&mut self, units: f32) {
        self.position += units * self.orientation.rotate_vector(Vector3::zero());
    }

    /// Rotate the entity.
    #[inline]
    pub fn look_at(&mut self, dir: Vector3<f32>, up: Vector3<f32>) {
        self.orientation = Quaternion::look_at(dir, up);
    }

    /// Add a component to the entity.
    ///
    /// Returns true if the component is unique for the entity.
    /// Any non unique components will be ignored and false will be returned.
    pub fn add<T: Any + Component>(&mut self, component: T) -> bool {
        if let None = self.component::<T>() {
            let wrapped = wrap!(component);

            wrapped.borrow_mut().subscribe(
                Rc::downgrade(&wrapped),
                &self.prop_container,
                &mut self.emitter);

            self.components.push(wrapped);

            return true;
        }

        return false;
    }

    /// Get a component by type.
    pub fn component<T: Any + Component>(&self) -> Option<&RefCell<T>> {
        for component in &self.components {
            if let Some(target) = component.downcast_ref::<RefCell<T>>() {
                return Some(target);
            }
        }

        return None
    }

    /// Emit an event.
    #[inline]
    pub fn emit(&mut self, event: Event) {
        self.emitter.emit(event, &mut self.prop_container);
    }

    /// Update the entity by emitting an `"update"` event.
    #[inline]
    pub fn update(&mut self) {
        self.emit(Event("update"));
    }
}

impl Deref for Entity {
    type Target = PropContainer;

    fn deref(&self) -> &PropContainer {
        &self.prop_container
    }
}

impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut PropContainer {
        &mut self.prop_container
    }
}

/// Holds common entity properties.
pub struct PropContainer {
    pub position: Point3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: f32
}

impl PropContainer {
    /// Create a new prop container.
    pub fn new() -> PropContainer {
        return PropContainer {
            position: Point3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::zero(),
            scale: 1.0
        };
    }

    /// Create a new prop container with values.
    pub fn from(position: Point3<f32>, orientation: Quaternion<f32>, scale: f32) -> PropContainer {
        return PropContainer {
            position: position,
            orientation: orientation,
            scale: scale
        };
    }
}
