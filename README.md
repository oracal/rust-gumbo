Rust Gumbo
==========

A Rust wrapper around google's c library [gumbo](https://github.com/google/gumbo-parser).

First and foremost it tries to provide idiomatic rust api. Secondly it tries to not have much added overhead on top of the c library.

Examples
--------

```rust
extern crate gumbo;
use gumbo::{Parser, Node, ElementNode};

fn dfs(node: &Node) {
    match *node {
        Element(ref element) => {
            for child_node in element.children().iter() {
                dfs(child_node)
            }
        },
        _ => {},
    }
}

let html: String = ...;
let parser = Parser::new();
let output = parser.parse(html.as_slice()).unwrap();
dfs(output.root());
```

Check out the examples folder for examples that match up to the c libraries examples.

TODO
----

* Add access to the parent of a node from that node to allow more complicated algorithms on the tree.
