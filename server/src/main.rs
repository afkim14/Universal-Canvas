//! |_________| is human nature.
//! You, and you alone, are responsible for your own actions.
//! With great power comes great responsibility.
//! You may choose to use your power to work with others and make something beautiful, or you may choose to use your power for selfish purposes.
//! Choose wisely.

// User Modules
pub mod canvas;
use canvas::*;
pub mod responder;
use responder::*;

extern crate sharing_is_caring;
use self::sharing_is_caring::*;

#[macro_use]
extern crate json;

extern crate ws;
use ws::WebSocket;

const LOCAL_HOST: &str = "127.0.0.1:8080";

type CanvasServer = Server<Canvas, Pixel, CanvasResponder>;

fn main() {
    let canvas = Canvas::new(50, 50, 10);
    let responder = CanvasResponder;
    let server = Server::new(canvas, responder);
    let ws = <WebSocket<CanvasServer>>::new(server).unwrap();
    ws.listen(LOCAL_HOST).unwrap();
}
