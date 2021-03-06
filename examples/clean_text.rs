extern crate gumbo;
extern crate libc;
use std::os::args;
use std::io::File;
use std::path::Path;
use gumbo::{Parser, Node, Element, Text, tag};
use std::os::set_exit_status;
use libc::consts::os::c95::EXIT_FAILURE;

fn clean_text(node: &Node) -> Vec<String> {
    let mut strings = Vec::new();
    match *node {
        Element(ref element) => {
            match element.tag() {
                tag::Script | tag::Style => {},
                _ => {
                    for child_node in element.children().iter() {
                        strings.push_all(clean_text(child_node).as_slice());
                    }
                }
            }
        },
        Text(ref text) => {
            strings.push(text.text());
        },
        _ => {}
    }
    strings
}

fn main() {
    if args().len() != 2u {
        println!("Usage: clean_text <html filename>");
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
    for cleaned_text in clean_text(output.root()).iter() {
        println!("{}", cleaned_text);
    }
}
