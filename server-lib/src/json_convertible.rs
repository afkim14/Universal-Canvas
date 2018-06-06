extern crate json;
use self::json::JsonValue;

pub trait AsJson {
    fn as_json(&self) -> JsonValue;
}

pub trait FromJson: Sized {
    fn from_json(json: &JsonValue) -> Option<Self>;
}
