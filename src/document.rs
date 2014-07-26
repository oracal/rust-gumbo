use ffi = super::ffi;
use super::node::Node;
use super::util::gumbo_vector_to_vector;
use std::string::raw::from_buf;

pub struct DocumentType<'a> {
    gumbo_document: &'a ffi::GumboDocument,
    children: Vec<Node<'a>>,
}

impl<'a> DocumentType<'a> {
    pub fn from_gumbo_document<'c>(document: &'c ffi::GumboDocument) -> DocumentType<'c> {
        DocumentType {
            gumbo_document: document,
            children: gumbo_vector_to_vector(&(*document).children).iter().filter_map(|&ptr| Node::from_gumbo_node(ptr)).collect(),
        }
    }

    pub fn children(&'a self) -> &'a Vec<Node<'a>> {
        &self.children
    }

    pub fn has_doctype(&self) -> bool {
        self.gumbo_document.has_doctype != 0
    }

    pub fn name(&self) -> String {
        unsafe {
            from_buf((*(self.gumbo_document)).name as *const u8)
        }
    }

    pub fn public_identifier(&self) -> String {
        unsafe {
            from_buf((*(self.gumbo_document)).public_identifier as *const u8)
        }
    }

    pub fn system_identifier(&self) -> String {
        unsafe {
            from_buf((*(self.gumbo_document)).system_identifier as *const u8)
        }
    }

    pub fn doc_type_quirks_mode(&self) -> ffi::GumboQuirksModeEnum {
        self.gumbo_document.doc_type_quirks_mode
    }
}
