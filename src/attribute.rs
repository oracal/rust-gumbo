use ffi = super::ffi;
use libc::{c_char, size_t};
use super::util::{buffer_to_option_string, cstr_to_option_string};

pub struct Attribute {
    gumbo_attribute: *mut ffi::GumboAttribute,
}

impl Attribute {
    pub fn from_gumbo_attribute(attribute: *mut ffi::GumboAttribute) -> Option<Attribute> {
        unsafe {
            if attribute.is_null() {
                None
            } else {
                Some(Attribute { gumbo_attribute: attribute })
            }
        }
    }

    pub fn namespace(self) -> ffi::GumboAttributeNamespaceEnum {
        unsafe {
            (*(self.gumbo_attribute)).attr_namespace
        }
    }

    pub fn name(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_attribute)).name)
        }
    }

    pub fn original_name(self) -> Option<String> {
        unsafe {
            buffer_to_option_string((*(self.gumbo_attribute)).original_name.data, (*(self.gumbo_attribute)).original_name.length)
        }
    }

    pub fn value(self) -> Option<String> {
        unsafe {
            cstr_to_option_string((*(self.gumbo_attribute)).value)
        }
    }

    pub fn original_value(self) -> Option<String> {
        unsafe {
            buffer_to_option_string((*(self.gumbo_attribute)).original_value.data, (*(self.gumbo_attribute)).original_value.length)
        }
    }

    pub fn name_start(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_attribute)).name_start
        }
    }

    pub fn name_end(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_attribute)).name_end
        }
    }

    pub fn value_start(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_attribute)).value_start
        }
    }

    pub fn value_end(self) -> ffi::GumboSourcePosition {
        unsafe {
            (*(self.gumbo_attribute)).value_end
        }
    }
}
