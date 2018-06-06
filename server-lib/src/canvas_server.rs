use server::*;
use canvas::*;
extern crate json;
use self::json::*;

struct CanvasResponder;
impl Responder<Canvas> for CanvasResponder {
    fn respond_to_request(&self, json_request: JsonValue, universe: SharedUniverse<Canvas>) -> JsonValue {
        unimplemented!()
    }
}

fn main() {
    let canvas = Canvas::new(50, 50, 50);
    let responder = CanvasResponder;
    let server = Server::new(canvas, "Canvas/1.0", responder);
}
