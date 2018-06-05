//! Contains implementation regarding our websocket server.
//!
//! We follow the paradigm outlined in the `ws` crate, and implement the proper trait methods.
//! See that crate for more details.
//!
//! On a high level, the singleton server owns the shared canvas, and gives a pointer to that shared canvas to client handlers.
//! Client handlers are created as each client establishes a connection with us.

extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use canvas::*;
use self::ws::{Message, Factory, Handler, Result, Sender};

type SharedCanvas = Arc<RwLock<Canvas>>;

/// A struct that contains the implementation for behavior associated with a single client, like when responding to a single client's requests.
/// For now, all client handlers are created equal because we treat all our clients equally, and so the fields in these objects are all the same.
/// However, this struct gives room for expansion.
///
/// See `CanvasServer::make_client_handler`.
pub struct ClientHandler {
    /// The `ws` stream associated with the single client.
    out: Sender,

    /// A pointer to the shared canvas across the server.
    canvas_lock: SharedCanvas,
}

/// A struct that contains implementation for behavior associated with the entire server, like creating `ClientHandler` events when a client tries to establish a connection initially.
pub struct CanvasServer {
    /// A pointer to the shared canvas across the server.
    canvas_lock: SharedCanvas,
}


impl CanvasServer {
    /// Creates a new server object, given a single canvas instance.
    /// The canvas is moved in since there is a shared canvas cross clients.
    pub fn new(canvas: Canvas) -> Self {
        CanvasServer { canvas_lock: Arc::new(RwLock::new(canvas)) }
    }

    /// Makes a default `ClientHandler` object.
    fn make_client_handler(&self, ws: Sender) -> ClientHandler {
        ClientHandler {
            out: ws,
            canvas_lock: self.canvas_lock.clone()
        }
    }
}


impl Factory for CanvasServer {
    type Handler = ClientHandler;

    fn connection_made(&mut self, ws: Sender) -> ClientHandler {
        // Returns default handler.
        self.make_client_handler(ws)
    }

    fn client_connected(&mut self, ws: Sender) -> ClientHandler {
        // Returns default handler.
        self.make_client_handler(ws)
    }
}

// REQUEST CONSTANTS

/// The expected key when clients ask for the entire board initially.
const RETRIEVE_BOARD :&str = "RETRIEVE_BOARD";

/// The expected key when clients change a single pixel.
const PIXEL_CHANGED: &str = "PIXEL_CHANGED";

impl Handler for ClientHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(msg_str) = msg.as_text() {
            if let Ok(json_data) = json::parse(msg_str) {
                println!("received request: {}", msg_str);
                match json_data["title"].as_str().unwrap() {
                    RETRIEVE_BOARD => {
                        let canvas_r = self.canvas_lock.read().unwrap();
                        let canvas_text = canvas_r.stringify();
                        return self.out.send(Message::Text(canvas_text));
                    },
                    PIXEL_CHANGED => {
                        // TODO: turn RETRIEVE BOARD into a json request so our parsing is consistent
                        // TODO: this is hell nesting. switch to convenience methods later
                        let new_pixel_json = &json_data["pixel_changed"];
                        let new_pixel_opt = Pixel::from_json(new_pixel_json);
                        if let Some(new_pixel) = new_pixel_opt {
                            let mut canvas_w = self.canvas_lock.write().unwrap();
                            canvas_w.update_pixel(new_pixel);
                            return self.out.broadcast(msg_str);
                        }
                    },
                    _ => {},
                }
            }
        }
        // TODO
        Ok(())
    }
}
