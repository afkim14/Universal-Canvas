use server::*;
use canvas::*;

fn main() {
    let canvas = Canvas::new(50, 50, 50);
    let responder = |request| {
        object!{}
    };
    let server = Server::new(canvas, "Canvas/1.0", responder);
}
