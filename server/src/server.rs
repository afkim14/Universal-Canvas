extern crate ws;
extern crate json;

use canvas::*;
use self::ws::{listen, Message, Handler, Result, Sender};
// use std::result::Result;

pub struct WSServer {
    canvas: Canvas,
    out: Sender,
}

impl WSServer {
    // pub fn new(canvas: Canvas) -> Self {
    //     WSServer { canvas }
    // }

    pub fn listen() {
        ws::listen("127.0.0.1:8080", |out| {
            // move |msg| {
            //     // out.send(msg)
            //     // self.handle_message(msg);
            //     println!("{:?}", msg);
            //     out.send(msg);
            //     Ok(())
            // }
            WSServer {
                canvas: Canvas::new(125, 125, 10),
                out,
            }
        }).unwrap();
    }

    // pub fn handle_message(&mut self, message: Message) {
    //     println!("{:?}", message);
    //     // mutex magic
    //     // update_canvas()
    //     // send()
    //     // unimplemented!();
    // }

    // sends changes to all clients
    pub fn send_changes(&mut self) {
        unimplemented!();
    }

    pub fn update_canvas(&mut self) {
        unimplemented!();
    }
}

// REQUEST CONSTANTS
const RETRIEVE_BOARD :&str = "RETRIEVE_BOARD";
const PIXEL_CHANGED: &str = "pixel_changed";

impl Handler for WSServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(msg_str) = msg.as_text() {
            println!("received request: {}", msg_str);
            match msg_str {
                RETRIEVE_BOARD => {
                    let canvas_text = self.canvas.stringify();
                    return self.out.send(Message::Text(canvas_text));
                },
                _ => {
                    // TODO: turn RETRIEVE BOARD into a json request so our parsing is consistent
                    // TODO: this is hell nesting. switch to convenience methods later
                    if let Ok(json_data) = json::parse(msg_str) {
                        let new_pixel_json = &json_data[PIXEL_CHANGED];
                        if new_pixel_json.is_object() {
                            let new_pixel_opt = Pixel::from_json(new_pixel_json);
                            if let Some(new_pixel) = new_pixel_opt {
                                self.canvas.update_pixel(new_pixel);
                            }
                        }
                    }
                },
            }
        }
        // TODO
        Ok(())
    }
}
