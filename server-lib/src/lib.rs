//! `sharing_is_caring` is a generic libary for writing WebSocket servers with an environment shared across multiple web clients.
//! Applications include a shared "canvas" where users can paint on, or a shared text document where users can edit lines.
//!
//! # Core Concepts
//! A universe represents the shared universe that the users interact with.
//!
//! An atom represents a small part of a universe.
//! For instance, in a shared universe of a canvas, with a grid of pixels, the atoms would be the pixels.
//!
//! See `universe` for more details.
//!
//! The idea is that users of this library can implement the traits in `universe`, which our pre-built `server` code can interface with, and then we can easily launch a WebSocket server for that universe.
//! Our server implementation already interfaces with the `ws` crate, so we can launch a server with that crate.
//!
//! # Sample Code (for now)
//! ```
//! struct Canvas { ... }
//! impl Universe for Canvas { ... }
//!
//! // ...
//!
//! let canvas = Canvas::new(50, 50, 50);
//! let responder = |request| {
//!     // Called each time when a web client sends a JSON request.
//!     // Take the request JSON object and create a JSON response
//!     return JsonValue::new();
//! };
//! let server = Server::new(canvas, "Canvas/1.0", responder);
//! // Launch a WS server with `server`
//! let ws = <WebSocket<Server>>::new(server).unwrap();
//! ws.listen("127.0.0.1:8080").unwrap();
//! ```
//!
//! For a shared text document, it might look something like this:
//! ```
//! struct Document { ... }
//! impl Universe for Document { ... }
//!
//! // ...
//!
//! let canvas = Document::new(50);
//! let responder = |request| {
//!     // Called each time when a web client sends a JSON request.
//!     // Take the request JSON object and create a JSON response
//!     return JsonValue::new();
//! };
//! let server = Server::new(canvas, "Canvas/1.0", responder);
//! // Launch a WS server with `server`
//! let ws = <WebSocket<Server>>::new(server).unwrap();
//! ws.listen("127.0.0.1:8080").unwrap();
//! ```

#[macro_use]
extern crate json;

pub mod server;
pub mod universe;
pub mod json_convertible;
// mod canvas; // TODO
// mod canvas_server;
