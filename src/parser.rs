use ffi = super::ffi;
use super::output::Output;

pub struct Parser {
    gumbo_options: ffi::GumboOptions,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { gumbo_options: ffi::kGumboDefaultOptions }
    }

    pub fn with_options(options: ffi::GumboOptions) -> Parser {
        Parser { gumbo_options: options }
    }

    pub fn parse(&self, body: &str) -> Option<Output> {
        unsafe {
            Output::from_gumbo_output(self.gumbo_options, body.with_c_str(|cstr| ffi::gumbo_parse_with_options(&(self.gumbo_options), cstr, body.len() as u64)))
        }
    }
}
