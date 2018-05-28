use std::net::{TcpListener};

// User Modules
mod canvas;
use canvas::*;
mod server;
use server::*;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // for stream in listener.incoming() {
    //     println!("new client\n");
    //     let stream = stream.unwrap();
    // }
    let mut server = WSServer::new(Canvas::new());
    server.listen();
}

fn send_canvas_to_clients(canvas: &Canvas) {
    unimplemented!();
}

fn update_canvas(pixels: &Vec<Pixel>) {
    unimplemented!();
}
