extern crate ws;
extern crate json;

use canvas::*;
use self::ws::{listen, Message, Factory, Handler, Result, Sender};
// use std::result::Result;


pub struct ClientHandler<'a> {
    out: Sender,
    is_connected: bool,
    parent : &'a CanvasServer,
}


pub struct CanvasServer {
    canvas:         Canvas,
}

impl<'a> ClientHandler<'a> {
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
        CanvasServer { canvas }
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


impl<'a> Factory for CanvasServer {

    type Handler = ClientHandler<'a>;

    fn connection_made(&mut self, ws: Sender) -> ClientHandler {
        ClientHandler {
            out : ws,
            is_connected : false,
            parent : self
        }
    }

    fn client_connected(&mut self, ws: Sender) -> ClientHandler {
        ClientHandler {
            out : ws,
            is_connected : true,   
            parent : self
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

impl<'a> Handler for ClientHandler<'a> {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        if let Ok(msg_str) = msg.as_text() {
            println!("received request: {}", msg_str);
            match msg_str {
                RETRIEVE_BOARD => {
                    let canvas_text = self.parent.canvas.stringify();
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
                                self.parent.canvas.update_pixel(new_pixel);
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
