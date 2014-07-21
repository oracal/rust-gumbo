#![crate_name = "gumbo"]
#![comment = "A rust wrapper for google's html parsing library"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate libc;

pub use parser::Parser;
pub use node::Node;
pub use element::ElementType;
pub use attribute::Attribute;
pub use document::DocumentType;
pub use text::TextType;
pub use node::{Document, Element, Text, Cdata, Comment, Whitespace};

pub mod ffi;
mod node;
mod output;
mod attribute;
mod document;
mod element;
mod text;
mod util;
mod parser;
