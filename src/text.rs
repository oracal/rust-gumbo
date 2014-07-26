use ffi = super::ffi;
use std::string::raw::{from_buf, from_buf_len};

pub struct TextType<'a> {
    gumbo_text: &'a ffi::GumboText,
}

impl<'a> TextType<'a> {
    pub fn from_gumbo_text(text: &'a ffi::GumboText) -> TextType<'a> {
        TextType{ gumbo_text: text }
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
