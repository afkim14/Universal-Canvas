extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use self::ws::{Sender, Factory, Handler, Message, Result};
use universe::Universe;

type SharedUniverse<U> = Arc<RwLock<U>>;

#[derive(Debug)]
pub struct Server<U> {
    universe: SharedUniverse<U>,
}

impl<U: Universe> Server<U> {
    pub fn new(universe: U) -> Self {
        Server {
            universe: Arc::new(RwLock::new(universe)),
        }
    }

    pub fn as_client_handler(&self, out: Sender) -> ClientHandler<U> {
        ClientHandler {
            out,
            universe: self.universe.clone()
        }
    }
}

impl<U: Universe> Factory for Server<U> {
    type Handler = ClientHandler<U>;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }

    fn client_connected(&mut self, out: Sender) -> Self::Handler {
        self.as_client_handler(out)
    }
}

pub struct ClientHandler<U> {
    out: Sender,
    universe: SharedUniverse<U>,
}

impl<U: Universe> Handler for ClientHandler<U> {
    fn on_message(&mut self, message: Message) -> Result<()> {
        unimplemented!()
    }
}
