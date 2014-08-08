use ffi = super::ffi;
use super::text::Text;
use super::document::Document;
use super::element::Element;

pub enum Node<'a> {
    DocumentNode(Document<'a>),
    ElementNode(Element<'a>),
    TextNode(Text<'a>),
    CdataNode(Text<'a>),
    CommentNode(Text<'a>),
    WhitespaceNode(Text<'a>),
}

impl<'a> Node<'a> {
    pub fn from_gumbo_node(node: *mut ffi::GumboNode) -> Option<Node<'a>> {
        unsafe {
            if node.is_null() {
                None
            } else {
                match (*node).node_type {
                    ffi::GUMBO_NODE_DOCUMENT   => Some(DocumentNode(Document::from_gumbo_document(node))),
                    ffi::GUMBO_NODE_ELEMENT    => Some(ElementNode(Element::from_gumbo_element(node))),
                    ffi::GUMBO_NODE_TEXT       => Some(TextNode(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_CDATA      => Some(CdataNode(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_COMMENT    => Some(CommentNode(Text::from_gumbo_text(node))),
                    ffi::GUMBO_NODE_WHITESPACE => Some(WhitespaceNode(Text::from_gumbo_text(node))),
                    _                          => None,
                }
            }
        }
    }
}
