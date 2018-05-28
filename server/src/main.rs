use std::net::{TcpListener};

// User Modules
mod canvas;
use canvas::*;

extern crate ws;

use ws::listen;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // for stream in listener.incoming() {
    //     println!("new client\n");
    //     let stream = stream.unwrap();
    // }

    listen("127.0.0.1:3012", |out| {
      move |msg| {
         out.send(msg)
      }
  }).unwrap()
}

fn send_canvas_to_clients(canvas: &Canvas) {
    unimplemented!();
}

fn update_canvas(pixels: &Vec<Pixel>) {
    unimplemented!();
}
