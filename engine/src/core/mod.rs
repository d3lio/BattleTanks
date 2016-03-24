//! The engine's core module.
//!
//! It contains any virtual world abstractions.

pub mod scene;
pub mod renderable;

pub use self::scene::Scene;
pub use self::renderable::Renderable;
