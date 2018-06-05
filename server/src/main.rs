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
    let server_rc = Rc::new(Cell::new(server));

    ws::listen(LOCAL_HOST, |out| {
        ClientHandler { out: out, server: server_rc.clone() }
    }).unwrap();
    //server.listen(LOCAL_HOST);
}
