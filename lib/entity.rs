use std::ops::Deref;
pub use transform::*;

/// An entity is anything that has spatial property.
#[derive(Debug)]
pub struct Entity<T> {
  pub object: T,
  pub transform: Transform
}

impl<T> Entity<T> {
  pub fn new(object: T, transform: Transform) -> Self {
    Entity {
      object: object,
      transform: transform
    }
  }
}

impl<T> Deref for Entity<T> {
  type Target = Transform;

  fn deref(&self) -> &Self::Target {
    &self.transform
  }
}

/// Trait of types that can see `Entity` updates.
pub trait EntityView {
  /// Called whenever the `Entity` gets its *object* changed.
  fn object_changed<T>(&mut self, entity: &Entity<T>);
  /// Called whenever the `Entity` gets its *transform* changed.
  fn transform_changed<T>(&mut self, entity: &Entity<T>);
}
