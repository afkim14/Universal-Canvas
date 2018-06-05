extern crate ws;
extern crate json;

use std::rc::Rc;
use canvas::*;
use self::ws::{listen, Message, Factory, Handler, Result, Sender};
// use std::result::Result;


pub struct ClientHandler {
    out: Sender,
    is_connected: bool,
    canvas : Rc<Canvas>,
}


pub struct CanvasServer {
    canvas: Rc<Canvas>,
}

impl<'a> ClientHandler {
    // pub fn handle_message(&mut self, message: Message) {
    //     println!("{:?}", message);
    //     // mutex magic
    //     // update_canvas()
    //     // send()
    //     // unimplemented!();
    // }

    // sends changes to all clients

}



impl CanvasServer {
    pub fn new(canvas: Canvas) -> Self {
        CanvasServer { canvas: Rc::new(canvas) }
    }

    pub fn listen(&mut self, host: &str) {
        ws::listen(host, |out| {
            self.connection_made(out)
        }).unwrap();
    }

    pub fn send_changes(&mut self) {
        unimplemented!();
    }

    pub fn update_canvas(&mut self) {
        unimplemented!();
    }

}


impl Factory for CanvasServer {
    type Handler = ClientHandler;

    fn connection_made(&mut self, ws: Sender) -> ClientHandler {
        ClientHandler {
            out : ws,
            is_connected : false,
            canvas: self.canvas.clone()
        }
    }

    fn client_connected(&mut self, ws: Sender) -> ClientHandler {
        ClientHandler {
            out : ws,
            is_connected : true,
            canvas: self.canvas.clone()
        }
    }

    fn connection_lost(&mut self, _: ClientHandler) {
        // handle
        unimplemented!();
    }
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
                    let canvas_text = Rc::get_mut(&mut self.canvas).unwrap().stringify();
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
                                Rc::get_mut(&mut self.canvas).unwrap().update_pixel(new_pixel);
                            }
                        }
                    }
                },
                _ => {},
            }
        }
        // TODO
        Ok(())
    }
}
