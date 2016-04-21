//! The engine's core module.
//!
//! It contains any virtual world abstractions.

mod color;
mod entity;
mod event_emitter;
mod scene;

pub use self::color::Color;

pub use self::entity::Entity;
pub use self::entity::cuboid::Cuboid;

pub use self::event_emitter::{Listener, Event, EventEmitter};

pub use self::scene::Scene;
pub use self::scene::camera::Camera;
pub use self::scene::composition::Composition;
pub use self::scene::renderable::Renderable;
