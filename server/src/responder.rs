extern crate sharing_is_caring;
use self::sharing_is_caring::*;
// use self::sharing_is_caring::server::*;
use canvas::{Canvas, Pixel};

// REQUEST CONSTANTS

/// The expected key when clients ask for the entire board initially.
const RETRIEVE_BOARD :&str = "RETRIEVE_BOARD";

/// The expected key when clients change a single pixel.
const PIXEL_CHANGED: &str = "PIXEL_CHANGED";

pub struct CanvasResponder;

impl Responder<Canvas> for CanvasResponder {
    fn respond_to_request(&self, json_request: JsonValue, universe: SharedUniverse<Canvas>) -> Response {
        match json_request["title"].as_str() {
            Some(RETRIEVE_BOARD) => {
                // let canvas_string: String;
                // {
                //     let canvas = self.universe.read().unwrap();
                //     canvas_string = canvas_r.stringify();
                // }
                // return self.out.send(Message::Text(canvas_text));
                let canvas = universe.read().unwrap();
                return Response::Reply(canvas.as_json());
            },
            Some(PIXEL_CHANGED) => {
                let new_pixel_json = &json_request["pixel_changed"];
                let new_pixel_opt = Pixel::from_json(new_pixel_json);
                if let Some(new_pixel) = new_pixel_opt {
                    let mut canvas = universe.write().unwrap();
                    canvas.update_atom(new_pixel);
                    // return self.out.broadcast(msg_str);
                    return Response::Broadcast(json_request.clone())
                }
            },
            _ => {},
        };
        Response::None
    }
}
