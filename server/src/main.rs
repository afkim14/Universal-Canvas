//! |_________| is human nature.
//! You, and you alone, are responsible for your own actions.
//! With great power comes great responsibility.
//! You may choose to use your power to work with others and make something beautiful, or you may choose to use your power for selfish purposes.
//! Choose wisely.

// User Modules
pub mod canvas;
use canvas::*;
pub mod server;
use server::*;

#[macro_use]
extern crate json;

extern crate ws;
use ws::WebSocket;

#![doc(html_root_url = "https://afkim14.github.io/canvas_finalproject/")]

const LOCAL_HOST: &str = "127.0.0.1:8080";

fn main() {
    let canvas = Canvas::new(50, 50, 10);
    let server = CanvasServer::new(canvas);
    let ws = <WebSocket<CanvasServer>>::new(server).unwrap();
    ws.listen(LOCAL_HOST).unwrap();
}
