use ffi = super::ffi;
use super::node::Node;
use super::util::gumbo_vector_to_vector;
use std::string::raw::from_buf;
use std::kinds::marker::ContravariantLifetime;

pub struct Document<'a> {
    node: *mut ffi::GumboNode,
    children: Vec<Node<'a>>,
    lt: ContravariantLifetime<'a>
}

impl<'a> Document<'a> {
    pub fn from_gumbo_document(node: *mut ffi::GumboNode) -> Document<'a> {
        unsafe {
            Document {
                node: node,
                children:
                    gumbo_vector_to_vector(&(*((*node).v.document())).children).
                    iter().
                    filter_map(|&ptr| Node::from_gumbo_node(ptr)).
                    collect(),
                lt: ContravariantLifetime,
            }
        }
    }

    pub fn children<'b>(&'b self) -> &'b [Node<'a>] {
        self.children.as_slice()
    }

    pub fn has_doctype(&self) -> bool {
        self.gumbo_document().has_doctype != 0
    }

    pub fn name(&self) -> String {
        unsafe {
            from_buf(self.gumbo_document().name as *const u8)
        }
    }

    pub fn public_identifier(&self) -> String {
        unsafe {
            from_buf(self.gumbo_document().public_identifier as *const u8)
        }
    }

    pub fn system_identifier(&self) -> String {
        unsafe {
            from_buf(self.gumbo_document().system_identifier as *const u8)
        }
    }

    pub fn doc_type_quirks_mode(&self) -> ffi::GumboQuirksModeEnum {
        self.gumbo_document().doc_type_quirks_mode
    }

    fn gumbo_document(&self) -> ffi::GumboDocument {
        unsafe {
            *((*(self.node)).v.document())
        }
    }
}
