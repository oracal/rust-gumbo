use ffi = super::ffi;
use super::node::Node;
use super::util::gumbo_vector_to_vector;
use std::string::raw::from_buf;
use std::kinds::marker::ContravariantLifetime;

pub struct Document<'a> {
    node: *mut ffi::GumboNode,
    lt: ContravariantLifetime<'a>
}

impl<'a> Eq for Document<'a> {}
impl<'a> PartialEq for Document<'a> {
    fn eq(&self, other: &Document<'a>) -> bool {
        self.node == other.node
    }
}

impl<'a> Document<'a> {
    pub fn from_gumbo_document(node: *mut ffi::GumboNode) -> Document<'a> {
        Document {
            node: node,
            lt: ContravariantLifetime,
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        unsafe {
            Node::from_gumbo_node((*self.node).parent)
        }
    }

    pub fn children(&self) -> Vec<Node<'a>> {
        gumbo_vector_to_vector(&self.gumbo_document().children).
            iter().
            filter_map(|&ptr| Node::from_gumbo_node(ptr)).
            collect()
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
