use ffi = super::ffi;
use super::text::TextType;
use super::document::DocumentType;
use super::element::ElementType;
use libc::size_t;

pub enum Content {
    Document(DocumentType),
    Element(ElementType),
    Text(TextType),
    Cdata(TextType),
    Comment(TextType),
    Whitespace(TextType),
}

pub struct Node {
    gumbo_node: *mut ffi::GumboNode,
    pub content: Content,
}

impl Node {
    pub fn from_gumbo_node(node: *mut ffi::GumboNode) -> Option<Node> {
        unsafe {
            if node.is_null() {
                None
            } else {
                match (*node).node_type {
                    ffi::GUMBO_NODE_DOCUMENT   => Some(Node {gumbo_node: node, content: Document(DocumentType::from_gumbo_document((*node).v.document()))}),
                    ffi::GUMBO_NODE_ELEMENT    => Some(Node {gumbo_node: node, content: Element(ElementType::from_gumbo_element((*node).v.element()))}),
                    ffi::GUMBO_NODE_TEXT       => Some(Node {gumbo_node: node, content: Text(TextType::from_gumbo_text((*node).v.text()))}),
                    ffi::GUMBO_NODE_CDATA      => Some(Node {gumbo_node: node, content: Cdata(TextType::from_gumbo_text((*node).v.text()))}),
                    ffi::GUMBO_NODE_COMMENT    => Some(Node {gumbo_node: node, content: Comment(TextType::from_gumbo_text((*node).v.text()))}),
                    ffi::GUMBO_NODE_WHITESPACE => Some(Node {gumbo_node: node, content: Whitespace(TextType::from_gumbo_text((*node).v.text()))}),
                    _                          => None,
                }
            }
        }
    }

    fn parent(self) -> Option<Node> {
        Node::from_gumbo_node(self.gumbo_node)
    }

    fn index_within_parent(self) -> size_t {
        unsafe {
            (*(self.gumbo_node)).index_within_parent
        }
    }

    fn parse_flags(self) -> ffi::GumboParseFlags {
        unsafe {
            (*(self.gumbo_node)).parse_flags
        }
    }
}
