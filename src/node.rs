use ffi = super::ffi;
use super::text::Text;
use super::document::Document;
use super::element::Element;

#[deriving(PartialEq, Eq)]
pub enum Node<'a> {
    Document(Document<'a>),
    Element(Element<'a>),
    Text(Text<'a>),
    Cdata(Text<'a>),
    Comment(Text<'a>),
    Whitespace(Text<'a>),
}

impl<'a> Node<'a> {
    pub fn from_gumbo_node(node: *mut ffi::GumboNode) -> Option<Node<'a>> {
        unsafe {
            if node.is_null() {
                None
            } else {
                match (*node).node_type {
                    ffi::GUMBO_NODE_DOCUMENT   => Some(Document(Document::from_gumbo_document(node))),
                    ffi::GUMBO_NODE_ELEMENT    => Some(Element(Element::from_gumbo_element(node))),
                    ffi::GUMBO_NODE_TEXT       => Some(Text(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_CDATA      => Some(Cdata(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_COMMENT    => Some(Comment(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_WHITESPACE => Some(Whitespace(Text::from_gumbo_text(node))),
                    _                          => None,
                }
            }
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        match *self {
            Document(document) => document.parent(),
            Element(element) => element.parent(),
            Text(text) => text.parent(),
            Cdata(text) => text.parent(),
            Comment(text) => text.parent(),
            Whitespace(text) => text.parent(),
        }
    }
}
