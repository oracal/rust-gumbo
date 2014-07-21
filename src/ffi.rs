use libc::{c_uint, c_char, size_t, c_void, c_int};

#[repr(C)]
pub struct GumboSourcePosition {
    pub line: c_uint,
    pub column: c_uint,
    pub offset: c_uint,
}

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

pub type GumboTag = c_uint;
pub static GUMBO_TAG_HTML: c_uint = 0;
pub static GUMBO_TAG_HEAD: c_uint = 1;
pub static GUMBO_TAG_TITLE: c_uint = 2;
pub static GUMBO_TAG_BASE: c_uint = 3;
pub static GUMBO_TAG_LINK: c_uint = 4;
pub static GUMBO_TAG_META: c_uint = 5;
pub static GUMBO_TAG_STYLE: c_uint = 6;
pub static GUMBO_TAG_SCRIPT: c_uint = 7;
pub static GUMBO_TAG_NOSCRIPT: c_uint = 8;
pub static GUMBO_TAG_TEMPLATE: c_uint = 9;
pub static GUMBO_TAG_BODY: c_uint = 10;
pub static GUMBO_TAG_ARTICLE: c_uint = 11;
pub static GUMBO_TAG_SECTION: c_uint = 12;
pub static GUMBO_TAG_NAV: c_uint = 13;
pub static GUMBO_TAG_ASIDE: c_uint = 14;
pub static GUMBO_TAG_H1: c_uint = 15;
pub static GUMBO_TAG_H2: c_uint = 16;
pub static GUMBO_TAG_H3: c_uint = 17;
pub static GUMBO_TAG_H4: c_uint = 18;
pub static GUMBO_TAG_H5: c_uint = 19;
pub static GUMBO_TAG_H6: c_uint = 20;
pub static GUMBO_TAG_HGROUP: c_uint = 21;
pub static GUMBO_TAG_HEADER: c_uint = 22;
pub static GUMBO_TAG_FOOTER: c_uint = 23;
pub static GUMBO_TAG_ADDRESS: c_uint = 24;
pub static GUMBO_TAG_P: c_uint = 25;
pub static GUMBO_TAG_HR: c_uint = 26;
pub static GUMBO_TAG_PRE: c_uint = 27;
pub static GUMBO_TAG_BLOCKQUOTE: c_uint = 28;
pub static GUMBO_TAG_OL: c_uint = 29;
pub static GUMBO_TAG_UL: c_uint = 30;
pub static GUMBO_TAG_LI: c_uint = 31;
pub static GUMBO_TAG_DL: c_uint = 32;
pub static GUMBO_TAG_DT: c_uint = 33;
pub static GUMBO_TAG_DD: c_uint = 34;
pub static GUMBO_TAG_FIGURE: c_uint = 35;
pub static GUMBO_TAG_FIGCAPTION: c_uint = 36;
pub static GUMBO_TAG_MAIN: c_uint = 37;
pub static GUMBO_TAG_DIV: c_uint = 38;
pub static GUMBO_TAG_A: c_uint = 39;
pub static GUMBO_TAG_EM: c_uint = 40;
pub static GUMBO_TAG_STRONG: c_uint = 41;
pub static GUMBO_TAG_SMALL: c_uint = 42;
pub static GUMBO_TAG_S: c_uint = 43;
pub static GUMBO_TAG_CITE: c_uint = 44;
pub static GUMBO_TAG_Q: c_uint = 45;
pub static GUMBO_TAG_DFN: c_uint = 46;
pub static GUMBO_TAG_ABBR: c_uint = 47;
pub static GUMBO_TAG_DATA: c_uint = 48;
pub static GUMBO_TAG_TIME: c_uint = 49;
pub static GUMBO_TAG_CODE: c_uint = 50;
pub static GUMBO_TAG_VAR: c_uint = 51;
pub static GUMBO_TAG_SAMP: c_uint = 52;
pub static GUMBO_TAG_KBD: c_uint = 53;
pub static GUMBO_TAG_SUB: c_uint = 54;
pub static GUMBO_TAG_SUP: c_uint = 55;
pub static GUMBO_TAG_I: c_uint = 56;
pub static GUMBO_TAG_B: c_uint = 57;
pub static GUMBO_TAG_U: c_uint = 58;
pub static GUMBO_TAG_MARK: c_uint = 59;
pub static GUMBO_TAG_RUBY: c_uint = 60;
pub static GUMBO_TAG_RT: c_uint = 61;
pub static GUMBO_TAG_RP: c_uint = 62;
pub static GUMBO_TAG_BDI: c_uint = 63;
pub static GUMBO_TAG_BDO: c_uint = 64;
pub static GUMBO_TAG_SPAN: c_uint = 65;
pub static GUMBO_TAG_BR: c_uint = 66;
pub static GUMBO_TAG_WBR: c_uint = 67;
pub static GUMBO_TAG_INS: c_uint = 68;
pub static GUMBO_TAG_DEL: c_uint = 69;
pub static GUMBO_TAG_IMAGE: c_uint = 70;
pub static GUMBO_TAG_IMG: c_uint = 71;
pub static GUMBO_TAG_IFRAME: c_uint = 72;
pub static GUMBO_TAG_EMBED: c_uint = 73;
pub static GUMBO_TAG_OBJECT: c_uint = 74;
pub static GUMBO_TAG_PARAM: c_uint = 75;
pub static GUMBO_TAG_VIDEO: c_uint = 76;
pub static GUMBO_TAG_AUDIO: c_uint = 77;
pub static GUMBO_TAG_SOURCE: c_uint = 78;
pub static GUMBO_TAG_TRACK: c_uint = 79;
pub static GUMBO_TAG_CANVAS: c_uint = 80;
pub static GUMBO_TAG_MAP: c_uint = 81;
pub static GUMBO_TAG_AREA: c_uint = 82;
pub static GUMBO_TAG_MATH: c_uint = 83;
pub static GUMBO_TAG_MI: c_uint = 84;
pub static GUMBO_TAG_MO: c_uint = 85;
pub static GUMBO_TAG_MN: c_uint = 86;
pub static GUMBO_TAG_MS: c_uint = 87;
pub static GUMBO_TAG_MTEXT: c_uint = 88;
pub static GUMBO_TAG_MGLYPH: c_uint = 89;
pub static GUMBO_TAG_MALIGNMARK: c_uint = 90;
pub static GUMBO_TAG_ANNOTATION_XML: c_uint = 91;
pub static GUMBO_TAG_SVG: c_uint = 92;
pub static GUMBO_TAG_FOREIGNOBJECT: c_uint = 93;
pub static GUMBO_TAG_DESC: c_uint = 94;
pub static GUMBO_TAG_TABLE: c_uint = 95;
pub static GUMBO_TAG_CAPTION: c_uint = 96;
pub static GUMBO_TAG_COLGROUP: c_uint = 97;
pub static GUMBO_TAG_COL: c_uint = 98;
pub static GUMBO_TAG_TBODY: c_uint = 99;
pub static GUMBO_TAG_THEAD: c_uint = 100;
pub static GUMBO_TAG_TFOOT: c_uint = 101;
pub static GUMBO_TAG_TR: c_uint = 102;
pub static GUMBO_TAG_TD: c_uint = 103;
pub static GUMBO_TAG_TH: c_uint = 104;
pub static GUMBO_TAG_FORM: c_uint = 105;
pub static GUMBO_TAG_FIELDSET: c_uint = 106;
pub static GUMBO_TAG_LEGEND: c_uint = 107;
pub static GUMBO_TAG_LABEL: c_uint = 108;
pub static GUMBO_TAG_INPUT: c_uint = 109;
pub static GUMBO_TAG_BUTTON: c_uint = 110;
pub static GUMBO_TAG_SELECT: c_uint = 111;
pub static GUMBO_TAG_DATALIST: c_uint = 112;
pub static GUMBO_TAG_OPTGROUP: c_uint = 113;
pub static GUMBO_TAG_OPTION: c_uint = 114;
pub static GUMBO_TAG_TEXTAREA: c_uint = 115;
pub static GUMBO_TAG_KEYGEN: c_uint = 116;
pub static GUMBO_TAG_OUTPUT: c_uint = 117;
pub static GUMBO_TAG_PROGRESS: c_uint = 118;
pub static GUMBO_TAG_METER: c_uint = 119;
pub static GUMBO_TAG_DETAILS: c_uint = 120;
pub static GUMBO_TAG_SUMMARY: c_uint = 121;
pub static GUMBO_TAG_MENU: c_uint = 122;
pub static GUMBO_TAG_MENUITEM: c_uint = 123;
pub static GUMBO_TAG_APPLET: c_uint = 124;
pub static GUMBO_TAG_ACRONYM: c_uint = 125;
pub static GUMBO_TAG_BGSOUND: c_uint = 126;
pub static GUMBO_TAG_DIR: c_uint = 127;
pub static GUMBO_TAG_FRAME: c_uint = 128;
pub static GUMBO_TAG_FRAMESET: c_uint = 129;
pub static GUMBO_TAG_NOFRAMES: c_uint = 130;
pub static GUMBO_TAG_ISINDEX: c_uint = 131;
pub static GUMBO_TAG_LISTING: c_uint = 132;
pub static GUMBO_TAG_XMP: c_uint = 133;
pub static GUMBO_TAG_NEXTID: c_uint = 134;
pub static GUMBO_TAG_NOEMBED: c_uint = 135;
pub static GUMBO_TAG_PLAINTEXT: c_uint = 136;
pub static GUMBO_TAG_RB: c_uint = 137;
pub static GUMBO_TAG_STRIKE: c_uint = 138;
pub static GUMBO_TAG_BASEFONT: c_uint = 139;
pub static GUMBO_TAG_BIG: c_uint = 140;
pub static GUMBO_TAG_BLINK: c_uint = 141;
pub static GUMBO_TAG_CENTER: c_uint = 142;
pub static GUMBO_TAG_FONT: c_uint = 143;
pub static GUMBO_TAG_MARQUEE: c_uint = 144;
pub static GUMBO_TAG_MULTICOL: c_uint = 145;
pub static GUMBO_TAG_NOBR: c_uint = 146;
pub static GUMBO_TAG_SPACER: c_uint = 147;
pub static GUMBO_TAG_TT: c_uint = 148;
pub static GUMBO_TAG_UNKNOWN: c_uint = 149;
pub static GUMBO_TAG_LAST: c_uint = 150;

pub type GumboAttributeNamespaceEnum = c_uint;
pub static GUMBO_ATTR_NAMESPACE_NONE: c_uint = 0;
pub static GUMBO_ATTR_NAMESPACE_XLINK: c_uint = 1;
pub static GUMBO_ATTR_NAMESPACE_XML: c_uint = 2;
pub static GUMBO_ATTR_NAMESPACE_XMLNS: c_uint = 3;

#[repr(C)]
pub struct GumboAttribute {
    pub attr_namespace: GumboAttributeNamespaceEnum,
    pub name: *const c_char,
    pub original_name: GumboStringPiece,
    pub value: *const c_char,
    pub original_value: GumboStringPiece,
    pub name_start: GumboSourcePosition,
    pub name_end: GumboSourcePosition,
    pub value_start: GumboSourcePosition,
    pub value_end: GumboSourcePosition,
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

pub type GumboNamespaceEnum = c_uint;
pub static GUMBO_NAMESPACE_HTML: c_uint = 0;
pub static GUMBO_NAMESPACE_SVG: c_uint = 1;
pub static GUMBO_NAMESPACE_MATHML: c_uint = 2;

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
    pub start_pos: GumboSourcePosition,
}

#[repr(C)]
pub struct GumboElement {
    pub children: GumboVector,
    pub tag: GumboTag,
    pub tag_namespace: GumboNamespaceEnum,
    pub original_tag: GumboStringPiece,
    pub original_end_tag: GumboStringPiece,
    pub start_pos: GumboSourcePosition,
    pub end_pos: GumboSourcePosition,
    pub attributes: GumboVector,
}

#[repr(C)]
pub struct NodeData {
    pub data: [u64, ..12u],
}

impl NodeData {
    pub fn document(&mut self) -> *mut GumboDocument {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn element(&mut self) -> *mut GumboElement {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn text(&mut self) -> *mut GumboText {
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
    pub static kGumboEmptySourcePosition: GumboSourcePosition;
    pub static kGumboEmptyString: GumboStringPiece;
    pub static kGumboEmptyVector: GumboVector;
    pub static kGumboDefaultOptions: GumboOptions;

    pub fn gumbo_string_equals(str1: *const GumboStringPiece, str2: *const GumboStringPiece) -> c_int;

    pub fn gumbo_string_equals_ignore_case(str1: *const GumboStringPiece, str2: *const GumboStringPiece) -> c_int;
    pub fn gumbo_vector_index_of(vector: *mut GumboVector, element: *mut c_void) -> c_int;
    pub fn gumbo_normalized_tagname(tag: GumboTag) -> *const c_char;
    pub fn gumbo_tag_from_original_text(text: *mut GumboStringPiece);
    pub fn gumbo_normalize_svg_tagname(tagname: *const GumboStringPiece) -> *const c_char;
    pub fn gumbo_tag_enum(tagname: *const c_char) -> GumboTag;
    pub fn gumbo_get_attribute(attrs: *const GumboVector, name: *const c_char) -> *mut GumboAttribute;
    pub fn gumbo_parse(buffer: *const c_char) -> *mut GumboOutput;
    pub fn gumbo_parse_with_options(options: *const GumboOptions, buffer: *const c_char, buffer_length: size_t) -> *mut GumboOutput;
    pub fn gumbo_destroy_output(options: *const GumboOptions, output: *mut GumboOutput);
}
