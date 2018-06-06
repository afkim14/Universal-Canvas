extern crate json;
use self::json::JsonValue;

pub trait Universe {
    type Atom;

    fn parse(json: &JsonValue) -> Self;
    fn stringify(&self) -> JsonValue;

    fn atom_key(&self) -> &str;
}
