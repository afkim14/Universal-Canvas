extern crate ws;
extern crate json;

use std::sync::{Arc, RwLock};
use self::ws::{Sender, Factory, Handler, Message, Result};
use std::marker::PhantomData;
use universe::{Universe, Atom};

type SharedUniverse<U> = Arc<RwLock<U>>;

#[derive(Debug)]
pub struct Server<U, A> {
    universe: SharedUniverse<U>,
    atom_phantom: PhantomData<A>,
}

impl<U: Universe<A>, A: Atom> Server<U, A> {
    pub fn new(universe: U) -> Self {
        Server {
            universe: Arc::new(RwLock::new(universe)),
            atom_phantom: PhantomData,
        }
    }

    pub fn as_client_handler(&self, out: Sender) -> ClientHandler<U, A> {
        ClientHandler {
            out,
            universe: self.universe.clone(),
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
    atom_phantom: PhantomData<A>,
}

impl<U: Universe<A>, A: Atom> Handler for ClientHandler<U, A> where U: Universe<A>, A: Atom {
    fn on_message(&mut self, message: Message) -> Result<()> {
        unimplemented!()
    }
}
