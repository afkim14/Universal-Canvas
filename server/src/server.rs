extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use canvas::*;
use self::ws::{Message, Factory, Handler, Result, Sender};

type SharedCanvas = Arc<RwLock<Canvas>>;

pub struct ClientHandler {
    out: Sender,
    canvas_lock: SharedCanvas,
}


pub struct CanvasServer {
    canvas_lock: SharedCanvas,
}


impl CanvasServer {
    pub fn new(canvas: Canvas) -> Self {
        CanvasServer { canvas_lock: Arc::new(RwLock::new(canvas)) }
    }

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
        self.make_client_handler(ws)
    }

    fn client_connected(&mut self, ws: Sender) -> ClientHandler {
        self.make_client_handler(ws)
    }
}

// REQUEST CONSTANTS
const RETRIEVE_BOARD :&str = "RETRIEVE_BOARD";
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
