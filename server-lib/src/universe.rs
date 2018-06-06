extern crate json;
use self::json::JsonValue;

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

pub trait Atom: JsonConvertible {
    fn id(&self) -> &usize;
}
