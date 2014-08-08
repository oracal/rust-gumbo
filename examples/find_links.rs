extern crate gumbo;
extern crate libc;
use std::os::args;
use std::io::File;
use std::path::Path;
use gumbo::{Parser, Node, ElementNode, tag};
use std::os::set_exit_status;
use libc::consts::os::c95::EXIT_FAILURE;

fn find_links(node: &Node) -> Vec<String> {
    let mut strings = Vec::new();
    match *node {
        ElementNode(ref element) => {
            match element.tag() {
                tag::A => match element.attributes().find_equiv(&"href").map(|&x| x.value()) {
                        Some(attribute_name) => strings.push(attribute_name),
                        None                 => {}
                    },
                _ => {}
            }
            for child_node in element.children().iter() {
                strings.push_all(find_links(child_node).as_slice());
            }
        },
        _ => {},
    }
    strings
}

fn main() {
    if args().len() != 2u {
        println!("Usage: find_links <html filename>");
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
    for link in find_links(output.root()).iter() {
        println!("{}", link);
    }
}
