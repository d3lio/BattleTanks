//! This module acts as a playground and thus will be removed in the future.

extern crate engine;
extern crate cgmath;

use cgmath::Vector3;

use engine::core::{Entity, Component, Data, Event, SubCallback};

use std::any::Any;
use std::cell::RefCell;

pub struct AntiClockwiseRotation {
    speed: f64,
    entity: Data
}

impl AntiClockwiseRotation {
    pub fn new(speed: f64) -> AntiClockwiseRotation {
        return AntiClockwiseRotation {
            speed: speed,
            entity: Data::null()
        };
    }
}

impl Component for AntiClockwiseRotation {
    fn init(&mut self, entity: &mut Entity, on: &SubCallback) {
        // We are sure that all the components and listeners will be
        // destroyed with the entity so this data pointer will never be invalid.
        self.entity = Data::from(entity);

        on(events!("rotate"), Box::new(
            |component: &Any, _: &Event, data: &Data| {
                let this = component.downcast_ref::<RefCell<Self>>().unwrap().borrow();
                let time = *data.to::<f64>();

                this.entity.to::<Entity>().look_at(
                    Vector3::new(
                        f64::cos(time * this.speed) as f32,
                        0.0,
                        f64::sin(time * this.speed) as f32),
                    Vector3::new(0.0, 1.0, 0.0));
            })
        );
    }
}
