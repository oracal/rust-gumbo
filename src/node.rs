use ffi = super::ffi;
use super::text::TextType;
use super::document::DocumentType;
use super::element::ElementType;
use libc::size_t;

pub enum Content<'a> {
    Document(DocumentType<'a>),
    Element(ElementType<'a>),
    Text(TextType<'a>),
    Cdata(TextType<'a>),
    Comment(TextType<'a>),
    Whitespace(TextType<'a>),
}

pub struct Node<'a> {
    gumbo_node: &'a ffi::GumboNode,
    pub content: Content<'a>,
}

impl<'a> Node<'a> {
    pub fn from_gumbo_node<'b>(node: &'b ffi::GumboNode) -> Option<Node<'b>> {
        match node.node_type {
            ffi::GUMBO_NODE_DOCUMENT   => Some(Node { gumbo_node: node, content: Document(DocumentType::from_gumbo_document(&*(*node).v.document()))}),
            ffi::GUMBO_NODE_ELEMENT    => Some(Node { gumbo_node: node, content: Element(ElementType::from_gumbo_element(&*(*node).v.element()))}),
            ffi::GUMBO_NODE_TEXT       => Some(Node { gumbo_node: node, content: Text(TextType::from_gumbo_text(&*(*node).v.text()))}),
            ffi::GUMBO_NODE_CDATA      => Some(Node { gumbo_node: node, content: Cdata(TextType::from_gumbo_text(&*(*node).v.text()))}),
            ffi::GUMBO_NODE_COMMENT    => Some(Node { gumbo_node: node, content: Comment(TextType::from_gumbo_text(&*(*node).v.text()))}),
            ffi::GUMBO_NODE_WHITESPACE => Some(Node { gumbo_node: node, content: Whitespace(TextType::from_gumbo_text(&*(*node).v.text()))}),
            _                          => None,
        }
    }

    pub fn index_within_parent(&self) -> size_t {
        self.gumbo_node.index_within_parent
    }

    pub fn parse_flags(&self) -> ffi::GumboParseFlags {
        self.gumbo_node.parse_flags
    }
}
