use ffi = super::ffi;
use super::node::Node;
use super::attribute::Attribute;
use super::util::{cstr_to_option_string, gumbo_vector_to_vector};

pub struct DocumentType {
    gumbo_document: *mut ffi::GumboDocument,
    children: Vec<Node>,
}

impl DocumentType {
    pub fn from_gumbo_document(document: *mut ffi::GumboDocument) -> DocumentType {
        unsafe {
            DocumentType {
                gumbo_document: document,
                children: gumbo_vector_to_vector((*document).children).iter().filter_map(|&ptr| Node::from_gumbo_node(ptr)).collect(),
            }
        }
    }

    pub fn children<'a>(&'a self) -> &'a Vec<Node> {
        &self.children
    }

    pub fn has_doctype(self) -> bool {
        unsafe {
            (*(self.gumbo_document)).has_doctype != 0
        }
    }

    pub fn name(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_document)).name)
        }
    }

    pub fn public_identifier(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_document)).public_identifier)
        }
    }

    pub fn system_identifier(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_document)).system_identifier)
        }
    }

    pub fn doc_type_quirks_mode(self) -> ffi::GumboQuirksModeEnum {
        unsafe {
            (*(self.gumbo_document)).doc_type_quirks_mode
        }
    }
}
