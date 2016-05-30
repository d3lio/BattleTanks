//! The engine's core module.
//!
//! It contains any virtual world abstractions and helper structures.

mod color;
mod data_ptr;
mod entity;
mod event_emitter;
mod scene;

pub mod input;

pub use self::color::Color;

pub use self::data_ptr::Data;

pub use self::entity::Entity;
pub use self::entity::component::{Component, SubCallback};
pub use self::entity::cuboid::Cuboid;

pub use self::event_emitter::{Event, EventEmitter, Listener};

pub use self::scene::Scene;
pub use self::scene::camera::Camera;
pub use self::scene::composition::Composition;
pub use self::scene::renderable::Renderable;
