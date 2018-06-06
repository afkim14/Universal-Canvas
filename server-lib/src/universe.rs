//! Contains traits for our universe-atom model.

use json_convertible::*;

/// Trait for objects that represent a universe in our model.
pub trait Universe<A>: AsJson {
    /// Called when we receive a request to update an atom in our universe.
    fn update_atom(&mut self, atom: A);
}

/// Trait for objects that represent atoms in this model.
/// For now, it just means we can convert to and from JSONs, but future extensions are possible.
pub trait Atom: AsJson + FromJson {}
