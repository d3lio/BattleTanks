extern crate cgmath;

pub mod cuboid;
pub mod component;

use self::cgmath::{
    VectorSpace, Rotation,
    Point3, Vector3, Quaternion
};

use core::{Data, EventEmitter, Listener};

use self::component::Component;

use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// Holds common virtual world object's properties and components.
pub struct Entity {
    pub position: Point3<f32>,
    pub orientation: Quaternion<f32>,
    pub scale: f32,
    emitter: EventEmitter<Any>,
    components: Vec<Rc<Any>>
}

impl Entity {
    /// Create a new entity.
    pub fn new() -> Entity {
        Entity::from(Point3::new(0.0, 0.0, 0.0), Quaternion::zero(), 1.0)
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
    /// Returns `Some` if the component is unique for the entity.
    /// Any non unique components will be ignored and `None` will be returned.
    pub fn add<T: Any + Component>(&mut self, component: T) -> Option<Rc<RefCell<T>>> {
        if let None = self.component::<T>() {
            // Wrap the component.
            let wrapped = wrap!(component);

            // Create a weak from the wrapper to clone later.
            let weak = Rc::downgrade(&wrapped);

            // Bypass rust safety checks.
            let this = Data::from(self);

            // Call the component's init method.
            wrapped.borrow_mut().init(self,
                &move |events, closure| {
                    // Create a listener from the given closure.
                    let listener = Listener::<Any>::new(weak.clone(), closure);
                    // Go through the events that the listener wants to listen for.
                    for event in events.into_iter() {
                        // Subscribe the listener to each of those events.
                        this.to::<Entity>().on(event, listener.clone());
                    }
                }
            );

            // Finally take in the wrapped component.
            self.components.push(wrapped.clone());

            return Some(wrapped);
        }

        return None;
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
}

impl Deref for Entity {
    type Target = EventEmitter<Any>;

    fn deref(&self) -> &Self::Target {
        &self.emitter
    }
}

impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.emitter
    }
}
