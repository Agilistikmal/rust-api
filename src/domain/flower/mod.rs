//! Flower Domain Module

pub mod errors;
pub mod flower_entity;

// Re-export the Flower entity and FlowerError
pub use flower_entity::Flower;
pub use errors::FlowerError;
