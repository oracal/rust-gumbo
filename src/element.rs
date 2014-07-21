use ffi = super::ffi;
use super::node::Node;
use super::attribute::Attribute;
use super::util::{cstr_to_option_string, buffer_to_option_string, gumbo_vector_to_vector};
use std::collections::HashMap;

pub struct ElementType {
    gumbo_element: *mut ffi::GumboElement,
    children: Vec<Node>,
    attributes: HashMap<String, Attribute>,
}

fn build_attributes(attributes: &[*mut ffi::GumboAttribute]) -> HashMap<String, Attribute> {
    unsafe {
        attributes.iter().
            filter(|&ptr| ptr.is_not_null()).
            map(|&ptr| (cstr_to_option_string((*ptr).name), Attribute::from_gumbo_attribute(ptr))).
            filter_map(|tuple| match tuple {
                (Some(name), Some(attr)) => Some((name, attr)),
                _                        => None,
            }).
            collect()
    }
}

impl ElementType {
    pub fn from_gumbo_element(element: *mut ffi::GumboElement) -> ElementType {
        unsafe {
            ElementType {
                gumbo_element: element,
                children: gumbo_vector_to_vector((*element).children).iter().filter_map(|&ptr| Node::from_gumbo_node(ptr)).collect(),
                attributes: build_attributes(gumbo_vector_to_vector((*element).attributes).as_slice()),
            }
        }
    }

    pub fn children<'a>(&'a self) -> &'a Vec<Node> {
        &self.children
    }

    pub fn tag(self) -> ffi::GumboTag {
        unsafe {
            (*(self.gumbo_element)).tag
        }
    }

    pub fn tag_namespace(self) -> ffi::GumboNamespaceEnum {
        unsafe {
            (*(self.gumbo_element)).tag_namespace
        }
    }

    pub fn original_tag(self) -> Option<String> {
        unsafe {
            buffer_to_option_string((*(self.gumbo_element)).original_tag.data, (*(self.gumbo_element)).original_tag.length)
        }
    }

    pub fn original_end_tag(self) -> Option<String> {
        unsafe {
            buffer_to_option_string((*(self.gumbo_element)).original_end_tag.data, (*(self.gumbo_element)).original_end_tag.length)
        }
    }

    pub fn start_pos(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_element)).start_pos
        }
    }

    pub fn end_pos(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_element)).end_pos
        }
    }

    pub fn attributes<'a>(&'a self) -> &'a HashMap<String, Attribute> {
        &self.attributes
    }
}
