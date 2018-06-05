// User Modules
mod canvas;
use canvas::*;
mod server;
use server::*;

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
