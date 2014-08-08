use ffi = super::ffi;
use std::string::raw::{from_buf, from_buf_len};
use super::util::SourcePosition;

pub struct Text<'a> {
    node: *mut ffi::GumboNode,
}

impl<'a> Text<'a> {
    pub fn from_gumbo_text(node: *mut ffi::GumboNode) -> Text<'a> {
        Text{
            node: node,
        }
    }

    pub fn text(&self) -> String {
        unsafe {
            from_buf(self.gumbo_text().text as *const u8)
        }
    }

    pub fn original_text(&self) -> String {
        unsafe {
            from_buf_len(self.gumbo_text().original_text.data as *const u8, self.gumbo_text().original_text.length as uint)
        }
    }

    pub fn start_pos(&self) -> SourcePosition {
        self.gumbo_text().start_pos
    }

    fn gumbo_text(&self) -> ffi::GumboText {
        unsafe {
            *((*(self.node)).v.text())
        }
    }
}
