extern crate gumbo;
use std::os::args;
use std::io::File;
use std::path::Path;
use gumbo::{Parser, Node, ElementNode, DocumentNode, tag};

fn find_links<'a>(node: &'a Node<'a>) -> Vec<String> {
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
        DocumentNode(ref document) => {
            for child_node in document.children().iter() {
                strings.push_all(find_links(child_node).as_slice());
            }
        },
        _ => {},
    }
    strings
}

fn main() {
    if args().len() != 2u {
        println!("please provide the filename");
        return;
    }

    let contents = File::open(&Path::new(args()[1].as_slice())).read_to_string().unwrap();
    let parser = Parser::new();
    let output = parser.parse(contents.as_slice()).unwrap();
    for link in find_links(&(output.root)).iter() {
        println!("{}", link);
    }
}
