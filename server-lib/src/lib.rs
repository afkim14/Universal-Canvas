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
//! # Sample Code
//! ```
//! struct Pixel { ... }
//! impl Atom for Pixel { ... }
//!
//! struct Canvas { ... }
//! impl Universe<Pixel> for Canvas { ... } // A canvas is a universe of pixels.
//!
//! struct CanvasResponder; // A zero-sized struct is fine.
//! impl Responder<Canvas> for CanvasResponder { ... } // Define the canvas's response behavior.
//!
//! fn main() {
//!     let canvas = Canvas::new(50, 50, 50);
//!     let responder = CanvasResponder;
//!     let server = Server::new(canvas, responder);
//!     // Launch a WS server with `server`
//!     server.listen("127.0.0.1:8080").unwrap();
//! }
//! ```
//!
//! For a shared text document, it might look something like this:
//! ```
//! struct Line { ... } // A line of text in our document.
//! impl Atom for Line { ... }
//!
//! struct Document { ... }
//! impl Universe<Line> for Document { ... } // A document is a universe of lines.
//!
//! struct DocumentResponder; // A zero-sized struct is fine.
//! impl Responder<Document> for DocumentResponder { ... } // Define the document's response behavior.
//!
//! fn main() {
//!     let document = Document::new();
//!     let responder = DocumentResponder;
//!     let server = Server::new(document, responder);
//!     // Launch a WS server with `server`
//!     server.listen("127.0.0.1:8080").unwrap();
//! }
//! ```

#[macro_use]
extern crate json;

pub mod server;
pub mod universe;
pub mod json_convertible;

pub use server::*;
pub use universe::*;
pub use json_convertible::*;

pub use self::json::JsonValue;
