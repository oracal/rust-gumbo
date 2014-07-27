use ffi = super::ffi;
use super::node::Node;
use super::attribute::Attribute;
use super::util::gumbo_vector_to_vector;
use std::collections::HashMap;
use super::tag::Tag;
use std::string::raw::{from_buf, from_buf_len};

pub struct Element<'a> {
    gumbo_node: &'a ffi::GumboNode,
    gumbo_element: &'a ffi::GumboElement,
    children: Vec<Node<'a>>,
    attributes: HashMap<String, Attribute<'a>>,
}

impl<'a> Element<'a> {
    pub fn from_gumbo_element<'b>(node: &'b ffi::GumboNode) -> Element<'b> {
        let element = node.v.element();
        let vector = gumbo_vector_to_vector(&(*element).attributes);
        Element {
            gumbo_node: node,
            gumbo_element: element,
            children: gumbo_vector_to_vector(&(*element).children).iter().filter_map(|&ptr| Node::from_gumbo_node(ptr)).collect(),
            attributes: Element::build_attributes(vector.as_slice()),
        }
    }

    fn build_attributes<'b>(attributes: &[&'b ffi::GumboAttribute]) -> HashMap<String, Attribute<'b>> {
        unsafe {
            attributes.iter().
                map(|&gumbo_attribute| (from_buf(gumbo_attribute.name as *const u8), Attribute::from_gumbo_attribute(gumbo_attribute))).collect()
        }
    }

    pub fn children(&'a self) -> &'a Vec<Node<'a>> {
        &self.children
    }

    pub fn tag(&self) -> Tag {
        self.gumbo_element.tag
    }

    pub fn tag_namespace(&self) -> ffi::GumboNamespaceEnum {
        self.gumbo_element.tag_namespace
    }

    pub fn original_tag(&self) -> String {
        unsafe {
            from_buf_len((*(self.gumbo_element).original_tag.data as *const u8), (*(self.gumbo_element)).original_tag.length as uint)
        }
    }

    pub fn original_end_tag(&self) -> String {
        unsafe {
            from_buf_len((*(self.gumbo_element)).original_end_tag.data as *const u8, (*(self.gumbo_element)).original_end_tag.length as uint)
        }
    }

    pub fn start_pos(&self) -> ffi::GumboSourcePosition {
        self.gumbo_element.start_pos
    }

    pub fn end_pos(&self) -> ffi::GumboSourcePosition {
        self.gumbo_element.end_pos
    }

    pub fn attributes(&'a self) -> &'a HashMap<String, Attribute<'a>> {
        &self.attributes
    }
}
