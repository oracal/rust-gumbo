use ffi = super::ffi;
use super::node::Node;

pub struct Output {
    gumbo_options: ffi::GumboOptions,
    gumbo_output: *mut ffi::GumboOutput,
    pub root: Node,
    pub document: Node,
}

impl Output {
    pub fn from_gumbo_output(options: ffi::GumboOptions, output: *mut ffi::GumboOutput) -> Option<Output> {
        unsafe {
            if output.is_null() || (*output).root.is_null() || (*output).document.is_null() {
                None
            } else {
                let root = Node::from_gumbo_node((*output).root);
                let document = Node::from_gumbo_node((*output).document);
                if root.is_none() || document.is_none() {
                    None
                } else {
                    Some(Output {
                        gumbo_options: options,
                        gumbo_output: output,
                        root: root.unwrap(),
                        document: document.unwrap(),
                    })
                }
            }
        }
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe {
            ffi::gumbo_destroy_output(&(self.gumbo_options), self.gumbo_output);
        }
    }
}
