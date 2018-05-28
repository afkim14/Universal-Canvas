extern crate ws;

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
                canvas: Canvas::new(125, 125),
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

impl Handler for WSServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{:?}", msg);
        unimplemented!()
    }
}
