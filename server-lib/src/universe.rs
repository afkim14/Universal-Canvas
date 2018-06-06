extern crate json;
use self::json::JsonValue;

use std::ops::{Index, IndexMut};

pub trait JsonConvertible {
    fn from_json(json: &JsonValue) -> Self;
    fn as_json(&self) -> JsonValue;
}

pub trait Universe<A>: JsonConvertible {
    fn atom_name_singular(&self) -> &str; // "pixel"
    // fn atom_name_plural(&self) -> &str; // "pixels"

    fn update_atom(&mut self, atom: A);

    fn atom_with_id(&self, id: usize) -> &A;
    fn atom_with_id_mut(&mut self, id: usize) -> &mut A;
}

impl<A> Index<usize> for Universe<A> where A: Atom {
    type Output = A;
    fn index(&self, id: usize) -> &Self::Output {
        self.atom_with_id(id)
    }
}

impl<A> IndexMut<usize> for Universe<A> where A: Atom {
    fn index_mut(&self, id: usize) -> &mut Self::Output {
        self.atom_with_id_mut(id)
    }
}

pub trait Atom: JsonConvertible {
    fn id(&self) -> &usize;
}
