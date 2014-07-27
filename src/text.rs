use ffi = super::ffi;
use std::string::raw::{from_buf, from_buf_len};

pub struct Text<'a> {
    gumbo_node: &'a ffi::GumboNode,
    gumbo_text: &'a ffi::GumboText,
}

impl<'a> Text<'a> {
    pub fn from_gumbo_text(node: &'a ffi::GumboNode) -> Text<'a> {
        Text{
            gumbo_node: node,
            gumbo_text: node.v.text(),
        }
    }

    pub fn text(&self) -> String {
        unsafe {
            from_buf((*(self.gumbo_text)).text as *const u8)
        }
    }

    pub fn original_text(&self) -> String {
        unsafe {
            from_buf_len((*(self.gumbo_text)).original_text.data as *const u8, (*(self.gumbo_text)).original_text.length as uint)
        }
    }

    pub fn start_pos(&self) -> ffi::GumboSourcePosition {
        self.gumbo_text.start_pos
    }
}
