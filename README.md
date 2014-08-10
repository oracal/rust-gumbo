Rust Gumbo
==========

A Rust wrapper around google's c library [gumbo](https://github.com/google/gumbo-parser).

First and foremost it tries to provide an idiomatic rust api around gumbo. Secondly, it tries to not add much overhead on top of the c library.

Example
-------

```rust
extern crate gumbo;
use gumbo::{Parser, Node, Element};

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

Check out the examples folder for examples that match up to the ones from the c library.

License
-------

MIT
