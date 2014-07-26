use ffi = super::ffi;
use super::node::Node;

pub struct Output<'a> {
    gumbo_options: ffi::GumboOptions,
    gumbo_output: *mut ffi::GumboOutput,
    pub root: Node<'a>,
    pub document: Node<'a>,
}

impl<'a> Output<'a> {
    pub fn from_gumbo_output(options: ffi::GumboOptions, output: *mut ffi::GumboOutput) -> Option<Output<'a>> {
        unsafe {
            if output.is_null() || (*output).root.is_null() || (*output).document.is_null() {
                None
            } else {
                match (Node::from_gumbo_node(&*(*output).root), Node::from_gumbo_node(&*(*output).document)) {
                    (Some(root), Some(document)) => Some(Output {
                        gumbo_options: options,
                        gumbo_output: output,
                        root: root,
                        document: document,
                    }),
                    _ => None
                }
            }
        }
    }
}

#[unsafe_destructor]
impl<'a> Drop for Output<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::gumbo_destroy_output(&(self.gumbo_options), self.gumbo_output);
        }
    }
}
