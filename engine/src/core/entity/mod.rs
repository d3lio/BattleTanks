extern crate cgmath;

pub mod cuboid;
pub mod component;

use self::cgmath::{
    VectorSpace, Rotation,
    Point3, Vector3, Quaternion
};

use core::{Data, Event, EventEmitter};

use self::component::Component;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

/// Holds common virtual world object's properties and components.
pub struct Entity {
    pub position: Point3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: f32,
    pub emitter: EventEmitter<Any>,
    components: Vec<Rc<Any>>
}

impl Entity {
    /// Create a new entity.
    pub fn new() -> Entity {
        return Entity {
            position: Point3::new(0.0, 0.0, 0.0),
            orientation: Quaternion::zero(),
            scale: 1.0,
            emitter: EventEmitter::new(),
            components: Vec::new()
        };
    }

    /// Create a new entity from properties.
    pub fn from(position: Point3<f32>, orientation: Quaternion<f32>, scale: f32) -> Entity {
        return Entity {
            position: position,
            orientation: orientation,
            scale: scale,
            emitter: EventEmitter::new(),
            components: Vec::new()
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

            wrapped.borrow_mut().subscribe(Rc::downgrade(&wrapped), self);

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

    /// Emit an event with some data.
    #[inline]
    pub fn emit(&mut self, event: Event, data: Data) {
        self.emitter.emit(event, data);
    }

    /// Emit an `update` event with the entity itself as data.
    #[inline]
    pub fn update(&mut self) {
        let data = Data::from(self);
        self.emit(Event("update"), data);
    }
}