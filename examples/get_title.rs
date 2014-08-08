extern crate gumbo;
extern crate libc;
use std::os::args;
use std::io::File;
use std::path::Path;
use gumbo::{Parser, Node, ElementNode, TextNode, tag};
use std::os::set_exit_status;
use libc::consts::os::c95::EXIT_FAILURE;

fn get_title<'a>(node: &'a Node<'a>) -> Option<String> {
    match *node {
        ElementNode(ref element) => {
            match element.tag() {
                tag::Title => {
                    for child_node in element.children().iter() {
                        match *child_node {
                            TextNode(text) => return Some(text.text()),
                            _ => {}
                        }
                    }
                    // title tag contains no text but is still there
                    return Some("".to_string())
                },
                _ => {},
            }
            for child_node in element.children().iter() {
                match get_title(child_node) {
                    maybe_title @ Some(_) => return maybe_title,
                    None => {},
                }
            }
        },
        _ => {},
    }
    None
}

fn main() {
    if args().len() != 2u {
        println!("Usage: get_title <html filename>");
        set_exit_status(EXIT_FAILURE as int);
        return;
    }

    let contents = match File::open(&Path::new(args()[1].as_slice())).read_to_string() {
        Ok(string) => string,
        Err(error) => {
            println!("{}", error.desc);
            set_exit_status(EXIT_FAILURE as int);
            return;
        }
    };

    let parser = Parser::new();
    let output = parser.parse(contents.as_slice()).unwrap();
    match get_title(output.root()) {
        Some(title) => println!("{}", title),
        None => println!("No title found")
    }
}
