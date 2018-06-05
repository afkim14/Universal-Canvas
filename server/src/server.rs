extern crate ws;
extern crate json;

use canvas::*;
use std::cell::Cell;
use std::rc::Rc;
use self::ws::{listen, Message, Handler, Result, Sender};
// use std::result::Result;

pub struct CanvasServer {
    canvas: Canvas,
}

pub struct ClientHandler {
    out: Sender,
    server: Rc<Cell<CanvasServer>>,
}

impl CanvasServer {
    // sends changes to all clients
    pub fn new(canvas: Canvas) -> Self {
        CanvasServer { canvas }
    }

    pub fn send_changes(&mut self) {
        unimplemented!();
    }

    pub fn update_canvas(&mut self) {
        unimplemented!();
    }
}

impl ClientHandler {
    // pub fn new(canvas: Canvas) -> Self {
    //     ClientHandler { canvas }
    // }

    // pub fn listen() {
    //     ws::listen("127.0.0.1:8080", |out| {
    //         // move |msg| {
    //         //     // out.send(msg)
    //         //     // self.handle_message(msg);
    //         //     println!("{:?}", msg);
    //         //     out.send(msg);
    //         //     Ok(())
    //         // }
    //         ClientHandler {
    //             canvas: Canvas::new(100, 60, 10),
    //             out,
    //         }
    //     }).unwrap();
    // }

    // pub fn handle_message(&mut self, message: Message) {
    //     println!("{:?}", message);
    //     // mutex magic
    //     // update_canvas()
    //     // send()
    //     // unimplemented!();
    // }
}

// REQUEST CONSTANTS
const RETRIEVE_BOARD :&str = "RETRIEVE_BOARD";
const PIXEL_CHANGED: &str = "PIXEL_CHANGED";

impl Handler for ClientHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(msg_str) = msg.as_text() {
            println!("received request: {}", msg_str);
            match msg_str {
                RETRIEVE_BOARD => {
                    let canvas_text = self.server.get_mut().canvas.stringify();
                    return self.out.send(Message::Text(canvas_text));
                },
                PIXEL_CHANGED => {
                    // TODO: turn RETRIEVE BOARD into a json request so our parsing is consistent
                    // TODO: this is hell nesting. switch to convenience methods later
                    if let Ok(json_data) = json::parse(msg_str) {
                        let new_pixel_json = &json_data[PIXEL_CHANGED];
                        if new_pixel_json.is_object() {
                            let new_pixel_opt = Pixel::from_json(new_pixel_json);
                            if let Some(new_pixel) = new_pixel_opt {
                                self.server.get_mut().canvas.update_pixel(new_pixel);
                            }
                        }
                    }

                    // TODO: Update all the other clients
                },
                _ => {},
            }
        }
        // TODO
        Ok(())
    }
}
