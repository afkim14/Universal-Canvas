//! Traits for objects that are convertible to and from JSON values.

extern crate json;
pub use self::json::JsonValue;

/// Trait for objects representable as a JSON value.
pub trait AsJson {
    /// Represents `self` as a JSON value.
    fn as_json(&self) -> JsonValue;
}

/// Trait for objects convertible from a JSON value.
pub trait FromJson: Sized {
    /// Constructs an instance from the JSON, if possible.
    fn from_json(json: &JsonValue) -> Option<Self>;
}
