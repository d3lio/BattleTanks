//! This module acts as a playground and thus will be removed in the future.

extern crate engine;
extern crate cgmath;

use cgmath::Vector3;

use engine::core::{Entity, Component, Event, Listener};

use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;

pub struct AntiClockwiseRotation {
    speed: f64
}

impl AntiClockwiseRotation {
    pub fn new(speed: f64) -> AntiClockwiseRotation {
        return AntiClockwiseRotation {
            speed: speed
        };
    }
}

impl Component for AntiClockwiseRotation {
    fn subscribe(&mut self, weak: Weak<RefCell<Self>>, entity: &mut Entity)
    {
        entity.emitter.on(Event("rotate"), Listener::<Any>::new(weak.clone(),
            |component, _, data| {
                let this = component.downcast_ref::<RefCell<Self>>().unwrap().borrow();
                let (entity, time) = *data.to::<(*mut Entity, f64)>();

                unsafe {
                    (*entity).look_at(
                        Vector3::new(
                            f64::cos(time * this.speed) as f32,
                            0.0,
                            f64::sin(time * this.speed) as f32),
                        Vector3::new(0.0, 1.0, 0.0));
                }
            }
        ));
    }
}
