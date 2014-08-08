use ffi = super::ffi;
use std::string::raw::{from_buf, from_buf_len};
use super::util::SourcePosition;
use std::kinds::marker::ContravariantLifetime;

#[repr(C)]
pub enum Namespace {
    None,
    Xlink,
    Xml,
    Xmlns,
}

pub struct Attribute<'a> {
    attribute: *mut ffi::GumboAttribute,
    lt: ContravariantLifetime<'a>
}

impl<'a> Attribute<'a> {
    pub fn from_gumbo_attribute(attribute: *mut ffi::GumboAttribute) -> Attribute<'a> {
        Attribute {
            attribute: attribute,
            lt: ContravariantLifetime,
        }
    }

    pub fn namespace(&self) -> Namespace {
        self.gumbo_attribute().attr_namespace
    }

    pub fn name(&self) -> String {
        unsafe {
            from_buf(self.gumbo_attribute().name as *const u8)
        }
    }

    pub fn original_name(&self) -> String {
        unsafe {
            from_buf_len(self.gumbo_attribute().original_name.data as *const u8, self.gumbo_attribute().original_name.length as uint)
        }
    }

    pub fn value(&self) -> String {
        unsafe {
            from_buf(self.gumbo_attribute().value as *const u8)
        }
    }

    pub fn original_value(&self) -> String {
        unsafe {
            from_buf_len(self.gumbo_attribute().original_value.data as *const u8, self.gumbo_attribute().original_value.length as uint)
        }
    }

    pub fn name_start(&self) -> SourcePosition {
        self.gumbo_attribute().name_start
    }

    pub fn name_end(&self) -> SourcePosition {
        self.gumbo_attribute().name_end
    }

    pub fn value_start(&self) -> SourcePosition {
        self.gumbo_attribute().value_start
    }

    pub fn value_end(&self) -> SourcePosition {
        self.gumbo_attribute().value_end
    }

    fn gumbo_attribute(&self) -> ffi::GumboAttribute {
        unsafe {
            *(self.attribute)
        }
    }
}
