extern crate gumbo;
extern crate libc;
use std::os::args;
use std::io::File;
use std::path::Path;
use gumbo::{Parser, Node, ElementNode};
use std::os::set_exit_status;
use libc::consts::os::c95::EXIT_FAILURE;

#[deriving(Show, Clone, PartialEq, Eq)]
struct Position {
    start: uint,
    end: uint,
}

fn positions_of_class(node: &Node, class_name: &str) -> Vec<Position> {
    let mut positions = Vec::new();
    match *node {
        ElementNode(ref element) => {
            match element.attributes().find_equiv(&"class") {
                Some(class_attribute) => {
                    match class_attribute.value().as_slice().find_str(class_name) {
                        Some(index) => {
                            let start = class_attribute.value_start().offset as uint + index + 1u;
                            positions.push(Position {
                                start: start,
                                end: start + class_name.len(),
                            })
                        },
                        None => {},
                    }
                },
                None => {},
            }
            for child_node in element.children().iter() {
                positions.push_all(positions_of_class(child_node, class_name).as_slice());
            }
        },
        _ => {},
    }
    positions
}

fn main() {
    if args().len() != 3u {
        println!("Usage: positions_of_class <html filename> <class name>");
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

    let class_name = args()[2].clone();

    let parser = Parser::new();
    let output = parser.parse(contents.as_slice()).unwrap();
    for position in positions_of_class(output.root(), class_name.as_slice()).iter() {
        println!("byte {} to byte {}", position.start, position.end);
        assert!(class_name.as_slice() == contents.as_slice().slice(position.start, position.end));
    }
}
