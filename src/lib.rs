#![feature(unsafe_destructor)]
#![crate_name = "gumbo"]
#![comment = "A rust wrapper for google's html parsing library"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate libc;

pub use parser::Parser;
pub use node::Node;
pub use element::Element;
pub use attribute::Attribute;
pub use document::Document;
pub use text::Text;
pub use node::{DocumentNode, ElementNode, TextNode, CdataNode, CommentNode, WhitespaceNode};

pub mod ffi;
mod node;
mod output;
mod attribute;
mod document;
mod element;
mod text;
mod util;
mod parser;
pub mod tag;
