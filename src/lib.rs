#![feature(unsafe_destructor)]
#![crate_name = "gumbo"]
#![comment = "A rust wrapper for google's html parsing library"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate libc;

pub use parser::Parser;
pub use node::Node;
pub use document::Document;
pub use element::{Element, ElementNamespace};
pub use text::Text;
pub use attribute::{Attribute, AttributeNamespace};
pub use node::{Document, Element, Text, Cdata, Comment, Whitespace};
pub use util::SourcePosition;
pub use output::Output;
pub use ffi::{GumboOptions, GumboAllocatorFunction, GumboDeallocatorFunction, GumboQuirksModeEnum};

#[allow(dead_code)]
mod ffi;
mod node;
mod output;
mod attribute;
mod document;
mod element;
mod text;
mod util;
mod parser;
pub mod tag;
