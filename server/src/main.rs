// User Modules
mod canvas;
use canvas::*;
mod server;
use server::*;

extern crate ws;
use ws::WebSocket;

const LOCAL_HOST: &str = "10.105.248.182:8080";

fn main() {
    let canvas = Canvas::new(100, 60, 10);
    let server = CanvasServer::new(canvas);
    let ws = <WebSocket<CanvasServer>>::new(server).unwrap();
    ws.listen(LOCAL_HOST).unwrap();
}
