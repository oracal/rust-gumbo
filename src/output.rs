use ffi = super::ffi;
use super::node::Node;
use std::kinds::marker::ContravariantLifetime;

struct OutputWrapper {
    options: ffi::GumboOptions,
    output: *mut ffi::GumboOutput,
}

impl Drop for OutputWrapper {
    fn drop(&mut self) {
        unsafe {
            ffi::gumbo_destroy_output(&(self.options), self.output);
        }
    }
}

#[allow(dead_code)]
pub struct Output<'a> {
    wrapper: OutputWrapper,
    root: Node<'a>,
    document: Node<'a>,
    lt: ContravariantLifetime<'a>,
}

impl<'a> Output<'a> {
    pub fn from_gumbo_output(options: ffi::GumboOptions, output: *mut ffi::GumboOutput) -> Option<Output<'a>> {
        unsafe {
            if output.is_null() || (*output).root.is_null() || (*output).document.is_null() {
                None
            } else {
                match (Node::from_gumbo_node((*output).root), Node::from_gumbo_node((*output).document)) {
                    (Some(root), Some(document)) => Some(Output {
                        wrapper: OutputWrapper {
                                     options: options,
                                     output: output
                                 },
                        root: root,
                        document: document,
                        lt: ContravariantLifetime,
                    }),
                    _ => None
                }
            }
        }
    }

    pub fn root(&'a self) -> &'a Node<'a> {
        &self.root
    }

    pub fn document(&'a self) -> &'a Node<'a> {
        &self.document
    }
}
