//! Contains implementation for a generic WebSocket server for objects that interface with traits in `universe`.

extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use std::rc::Rc;
use self::ws::{Sender, Factory, Handler, Message, Result};
use std::marker::PhantomData;

use self::json::*;

use universe::{Universe, Atom};

pub type SharedUniverse<U> = Arc<RwLock<U>>;

/// A trait for objects that can respond to requests from web clients.
pub trait Responder<U> {
    /// Takes the request JSON and a mutable reference to the universe, mutates accordingly, and outputs a response JSON.
    fn respond_to_request(&self, json_request: JsonValue, universe: SharedUniverse<U>) -> Response;
}

#[derive(Debug)]
pub enum Response {
    None,
    Reply(JsonValue),
    Broadcast(JsonValue),
}

/// The server object that defines behavior during the lifetime of our program.
pub struct Server<U, A, R> {
    universe: SharedUniverse<U>,
    /// The function called for every request that we receive from a client.
    responder: Rc<R>,
    /// Zero-size data so we can make our generics work.
    atom_phantom: PhantomData<A>,
}

impl<U, A, R> Server<U, A, R>
    where U: Universe<A>, A: Atom, R: Responder<U> {
    /// Creates a new server with the given universe, name of our server, and a function to generate a response whenever we receive a request.
    pub fn new(universe: U, responder: R) -> Self {
        Server {
            universe: Arc::new(RwLock::new(universe)),
            responder: Rc::new(responder),
            atom_phantom: PhantomData,
        }
    }

    /// Creates a `ClientHandler` instance from the object.
    pub fn as_client_handler(&self, out: Sender) -> ClientHandler<U, A, R> {
        ClientHandler {
            out,
            universe: self.universe.clone(),
            responder: self.responder.clone(),
            atom_phantom: PhantomData,
        }
    }

    pub fn listen(self, host: &str) -> ws::Result<()> {
        let ws = <ws::WebSocket<Server<U, A, R>>>::new(self)?;
        ws.listen(host)?;
        Ok(())
    }
}

impl<U, A, R> Factory for Server<U, A, R>
    where U: Universe<A>, A: Atom, R: Responder<U> {
    type Handler = ClientHandler<U, A, R>;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }

    fn client_connected(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }
}

/// A struct that contains the implementation for behavior associated with a single client, like when responding to a single client's requests.
/// For now, all client handlers are created equal because we treat all our clients equally, and so the fields in these objects are all the same.
/// However, this struct gives room for expansion.
///
/// See `Server::as_client_handler`.
pub struct ClientHandler<U, A, R> {
    out: Sender,
    universe: SharedUniverse<U>,
    responder: Rc<R>,
    atom_phantom: PhantomData<A>,
}

impl<U, A, R> ClientHandler<U, A, R>
    where U: Universe<A>, A: Atom, R: Responder<U> {
    fn send_error_message(&self) -> Result<()> {
        let message = object!{
            "error" => "Bad request.",
        };
        let message_str = json::stringify(message);
        self.out.send(Message::Text(message_str))
    }
}

impl<U, A, R> Handler for ClientHandler<U, A, R>
    where U: Universe<A>, A: Atom, R: Responder<U> {
    fn on_message(&mut self, message: Message) -> Result<()> {
        match message.as_text()
            .map(|s| json::parse(s)) {
            Ok(Ok(request_json)) => {
                let response = self.responder.respond_to_request(request_json, self.universe.clone());
                // println!("Generated response {:?}", response);
                use self::Response::*;
                match response {
                    Reply(reply) => {
                        let response_str = json::stringify(reply);
                        self.out.send(Message::Text(response_str))
                    },
                    Broadcast(broadcast) => {
                        let response_str = json::stringify(broadcast);
                        self.out.broadcast(Message::Text(response_str))
                    },
                    _ => Ok(()),
                }
            },
            _ => self.send_error_message(),
        }
    }
}
