use libc::{c_uint, c_char, size_t, c_void, c_int};
use super::tag::Tag;
use super::element;
use super::attribute;
use super::util::SourcePosition;

#[repr(C)]
pub struct GumboStringPiece {
    pub data: *const c_char,
    pub length: size_t,
}

#[repr(C)]
pub struct GumboVector {
    pub data: *mut *mut c_void,
    pub length: c_uint,
    pub capacity: c_uint,
}


#[repr(C)]
pub struct GumboAttribute {
    pub attr_namespace: attribute::Namespace,
    pub name: *const c_char,
    pub original_name: GumboStringPiece,
    pub value: *const c_char,
    pub original_value: GumboStringPiece,
    pub name_start: SourcePosition,
    pub name_end: SourcePosition,
    pub value_start: SourcePosition,
    pub value_end: SourcePosition,
}

pub type GumboNodeType = c_uint;
pub static GUMBO_NODE_DOCUMENT: c_uint = 0;
pub static GUMBO_NODE_ELEMENT: c_uint = 1;
pub static GUMBO_NODE_TEXT: c_uint = 2;
pub static GUMBO_NODE_CDATA: c_uint = 3;
pub static GUMBO_NODE_COMMENT: c_uint = 4;
pub static GUMBO_NODE_WHITESPACE: c_uint = 5;

pub type GumboQuirksModeEnum = c_uint;
pub static GUMBO_DOCTYPE_NO_QUIRKS: c_uint = 0;
pub static GUMBO_DOCTYPE_QUIRKS: c_uint = 1;
pub static GUMBO_DOCTYPE_LIMITED_QUIRKS: c_uint = 2;

pub type GumboParseFlags = c_uint;
pub static GUMBO_INSERTION_NORMAL: c_uint = 0;
pub static GUMBO_INSERTION_BY_PARSER: c_uint = 1;
pub static GUMBO_INSERTION_IMPLICIT_END_TAG: c_uint = 2;
pub static GUMBO_INSERTION_IMPLIED: c_uint = 8;
pub static GUMBO_INSERTION_CONVERTED_FROM_END_TAG: c_uint = 16;
pub static GUMBO_INSERTION_FROM_ISINDEX: c_uint = 32;
pub static GUMBO_INSERTION_FROM_IMAGE: c_uint = 64;
pub static GUMBO_INSERTION_RECONSTRUCTED_FORMATTING_ELEMENT: c_uint = 128;
pub static GUMBO_INSERTION_ADOPTION_AGENCY_CLONED: c_uint = 256;
pub static GUMBO_INSERTION_ADOPTION_AGENCY_MOVED: c_uint = 512;
pub static GUMBO_INSERTION_FOSTER_PARENTED: c_uint = 1024;

#[repr(C)]
pub struct GumboDocument {
    pub children: GumboVector,
    pub has_doctype: c_int,
    pub name: *const c_char,
    pub public_identifier: *const c_char,
    pub system_identifier: *const c_char,
    pub doc_type_quirks_mode: GumboQuirksModeEnum,
}

#[repr(C)]
pub struct GumboText {
    pub text: *const c_char,
    pub original_text: GumboStringPiece,
    pub start_pos: SourcePosition,
}

#[repr(C)]
pub struct GumboElement {
    pub children: GumboVector,
    pub tag: Tag,
    pub tag_namespace: element::Namespace,
    pub original_tag: GumboStringPiece,
    pub original_end_tag: GumboStringPiece,
    pub start_pos: SourcePosition,
    pub end_pos: SourcePosition,
    pub attributes: GumboVector,
}

#[repr(C)]
pub struct NodeData {
    pub data: [u64, ..12u],
}

impl NodeData {
    pub fn document(&self) -> &GumboDocument {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn element(&self) -> &GumboElement {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn text(&self) -> &GumboText {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[repr(C)]
pub struct GumboNode {
    pub node_type: GumboNodeType,
    pub parent: *mut GumboNode,
    pub index_within_parent: size_t,
    pub parse_flags: GumboParseFlags,
    pub v: NodeData,
}

pub type GumboAllocatorFunction = extern fn (arg1: *mut c_void, arg2: size_t) -> *mut c_void;
pub type GumboDeallocatorFunction = extern fn (arg1: *mut c_void, arg2: *mut c_void);

#[repr(C)]
pub struct GumboOptions {
    pub allocator: GumboAllocatorFunction,
    pub deallocator: GumboDeallocatorFunction,
    pub userdata: *mut c_void,
    pub tab_stop: c_int,
    pub stop_on_first_error: c_int,
    pub max_errors: c_int,
}

#[repr(C)]
pub struct GumboOutput {
    pub document: *mut GumboNode,
    pub root: *mut GumboNode,
    pub errors: GumboVector,
}

#[link(name = "gumbo")]
extern {
    pub static kGumboEmptySourcePosition: SourcePosition;
    pub static kGumboEmptyString: GumboStringPiece;
    pub static kGumboEmptyVector: GumboVector;
    pub static kGumboDefaultOptions: GumboOptions;

    pub fn gumbo_string_equals(str1: *const GumboStringPiece, str2: *const GumboStringPiece) -> c_int;

    pub fn gumbo_string_equals_ignore_case(str1: *const GumboStringPiece, str2: *const GumboStringPiece) -> c_int;
    pub fn gumbo_vector_index_of(vector: *mut GumboVector, element: *mut c_void) -> c_int;
    pub fn gumbo_normalized_tagname(tag: Tag) -> *const c_char;
    pub fn gumbo_tag_from_original_text(text: *mut GumboStringPiece);
    pub fn gumbo_normalize_svg_tagname(tagname: *const GumboStringPiece) -> *const c_char;
    pub fn gumbo_tag_enum(tagname: *const c_char) -> Tag;
    pub fn gumbo_get_attribute(attrs: *const GumboVector, name: *const c_char) -> *mut GumboAttribute;
    pub fn gumbo_parse(buffer: *const c_char) -> *mut GumboOutput;
    pub fn gumbo_parse_with_options(options: *const GumboOptions, buffer: *const c_char, buffer_length: size_t) -> *mut GumboOutput;
    pub fn gumbo_destroy_output(options: *const GumboOptions, output: *mut GumboOutput);
}
