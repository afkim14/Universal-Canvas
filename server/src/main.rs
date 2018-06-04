use std::net::{TcpListener};

// User Modules
mod canvas;
use canvas::*;
mod server;
use server::*;

extern crate ws;

use ws::listen;


const LOCAL_HOST: &str = "127.0.0.1:8080";

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // for stream in listener.incoming() {
    //     println!("new client\n");
    //     let stream = stream.unwrap();
    // }
    let mut canvas = Canvas::new(100, 60, 10);
    let server = CanvasServer::new(canvas);
    server.listen(LOCAL_HOST);
}

// fn send_canvas_to_clients(canvas: &Canvas) {
//     unimplemented!();
// }

// fn update_canvas(pixels: &Vec<Pixel>) {
//     unimplemented!();
// }
