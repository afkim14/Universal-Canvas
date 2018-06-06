extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use std::rc::Rc;
use self::ws::{Sender, Factory, Handler, Message, Result};
use std::marker::PhantomData;

use self::json::*;

use universe::{Universe, Atom};

type SharedUniverse<U> = Arc<RwLock<U>>;

pub type SharedResponder = Rc<Fn(JsonValue) -> JsonValue>;

pub struct Server<U, A> {
    universe: SharedUniverse<U>,
    application_name: &'static str,
    responder: SharedResponder,
    atom_phantom: PhantomData<A>,
}

impl<U: Universe<A>, A: Atom> Server<U, A> {
    pub fn new<R: 'static>(universe: U, application_name: &'static str, responder: R) -> Self where R: Fn(JsonValue) -> JsonValue {
        Server {
            universe: Arc::new(RwLock::new(universe)),
            application_name,
            responder: Rc::new(responder),
            atom_phantom: PhantomData,
        }
    }

    pub fn as_client_handler(&self, out: Sender) -> ClientHandler<U, A> {
        ClientHandler {
            out,
            universe: self.universe.clone(),
            responder: self.responder.clone(),
            application_name: self.application_name,
            atom_phantom: PhantomData,
        }
    }
}

impl<U: Universe<A>, A: Atom> Factory for Server<U, A> {
    type Handler = ClientHandler<U, A>;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }

    fn client_connected(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }
}

pub struct ClientHandler<U, A> {
    out: Sender,
    universe: SharedUniverse<U>,
    responder: SharedResponder,
    application_name: &'static str,
    atom_phantom: PhantomData<A>,
}

impl<'a, U: Universe<A>, A: Atom> ClientHandler<U, A> {
    fn send_error_message(&self) -> Result<()> {
        let message = object!{
            "application" => self.application_name,
            "error" => "Bad request.",
        };
        let message_str = json::stringify(message);
        self.out.send(Message::Text(message_str))
    }
}

impl<'a, U: Universe<A>, A: Atom> Handler for ClientHandler<U, A> {
    fn on_message(&mut self, message: Message) -> Result<()> {
        match message.as_text()
            .map(|s| json::parse(s)) {
            Ok(Ok(request_json)) => {
                let response = (self.responder)(request_json);
                let response_str = json::stringify(response);
                self.out.send(Message::Text(response_str))
            },
            _ => self.send_error_message(),
        }
    }
}
