use ffi = super::ffi;
use super::util::{buffer_to_option_string, cstr_to_option_string};

pub struct TextType {
    gumbo_text: *mut ffi::GumboText,
}

impl TextType {
    pub fn from_gumbo_text(text: *mut ffi::GumboText) -> TextType {
        TextType{ gumbo_text: text }
    }


    fn text(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_text)).text)
        }
    }

    fn original_text(self) -> Option<String> {
        unsafe {
            buffer_to_option_string((*(self.gumbo_text)).original_text.data, (*(self.gumbo_text)).original_text.length)
        }
    }

    fn start_pos(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_text)).start_pos
        }
    }
}
