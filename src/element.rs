use ffi = super::ffi;
use super::node::Node;
use super::attribute::Attribute;
use super::util::{SourcePosition, gumbo_vector_to_vector};
use std::collections::HashMap;
use super::tag::Tag;
use std::string::raw::{from_buf, from_buf_len};
use std::kinds::marker::ContravariantLifetime;

#[repr(C)]
pub enum Namespace {
    Html,
    Svg,
    MathML,
}

pub struct Element<'a> {
    node: *mut ffi::GumboNode,
    children: Vec<Node<'a>>,
    attributes: HashMap<String, Attribute<'a>>,
    lt: ContravariantLifetime<'a>,
}

impl<'a> Element<'a> {
    pub fn from_gumbo_element(node: *mut ffi::GumboNode) -> Element<'a> {
        unsafe {
            let element = (*node).v.element();
            Element {
                node: node,
                children: gumbo_vector_to_vector(&(*element).children).iter().filter_map(|&ptr| Node::from_gumbo_node(ptr)).collect(),
                attributes: Element::build_attributes(gumbo_vector_to_vector(&(*element).attributes).as_slice()),
                lt: ContravariantLifetime,
            }
        }
    }

    fn build_attributes<'b>(attributes: &[*mut ffi::GumboAttribute]) -> HashMap<String, Attribute<'b>> {
        unsafe {
            attributes.iter().
                map(|&attribute| (from_buf((*attribute).name as *const u8), Attribute::from_gumbo_attribute(attribute))).collect()
        }
    }

    pub fn children(&'a self) -> &'a Vec<Node<'a>> {
        &self.children
    }

    pub fn tag(&self) -> Tag {
        self.gumbo_element().tag
    }

    pub fn tag_namespace(&self) -> Namespace {
        self.gumbo_element().tag_namespace
    }

    pub fn original_tag(&self) -> String {
        unsafe {
            from_buf_len(self.gumbo_element().original_tag.data as *const u8, self.gumbo_element().original_tag.length as uint)
        }
    }

    pub fn original_end_tag(&self) -> String {
        unsafe {
            from_buf_len(self.gumbo_element().original_end_tag.data as *const u8, self.gumbo_element().original_end_tag.length as uint)
        }
    }

    pub fn start_pos(&self) -> SourcePosition {
        self.gumbo_element().start_pos
    }

    pub fn end_pos(&self) -> SourcePosition {
        self.gumbo_element().end_pos
    }

    pub fn attributes(&'a self) -> &'a HashMap<String, Attribute<'a>> {
        &self.attributes
    }

    fn gumbo_element(&self) -> ffi::GumboElement {
        unsafe {
            *((*(self.node)).v.element())
        }
    }
}
