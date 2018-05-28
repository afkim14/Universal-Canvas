extern crate ws;

use canvas::*;
use self::ws::{listen, Message};
use std::result::Result;

pub struct WSServer {
    canvas: Canvas,
}

impl WSServer {
    pub fn new(canvas: Canvas) -> Self {
        WSServer { canvas }
    }

    pub fn listen(&mut self) {
        ws::listen("127.0.0.1:8080", |out| {
            move |msg| {
                // out.send(msg)
                // self.handle_message(msg);
                println!("{:?}", msg);
                Result::Ok(())
            }
        }).unwrap();
    }

    pub fn handle_message(&mut self, message: Message) {
        println!("{:?}", message);
        // mutex magic
        // update_canvas()
        // send()
        // unimplemented!();
    }

    // sends changes to all clients
    pub fn send_changes(&mut self) {
        unimplemented!();
    }

    pub fn update_canvas(&mut self) {
        unimplemented!();
    }
}
