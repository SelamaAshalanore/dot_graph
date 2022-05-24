# dot_graph

A library for generating Graphviz DOT language files.
The code is forked from [dot-rust](https://github.com/przygienda/dot-rust) with new API for easier to use.

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Usage
The very basic three parts of a DOT file and in this library is `Graph`,
`Node` and `Edge`. `Graph` is the entry point of the library. You could
generate an empty graph .dot file by simply use:

```rust
use dot_graph::{Graph, Kind};

let graph = Graph::new("empty_graph", Kind::Digraph);

let dot_string = graph.to_dot_string().unwrap();

assert_eq!(dot_string,
   r#"digraph empty_graph {
}
//!"#);
```

In order to add some basic nodes and edges:

```rust
use dot_graph::{Graph, Kind, Node, Edge};

let mut graph = Graph::new("single_edge", Kind::Digraph);

graph.add_node(Node::new("N0"));
graph.add_node(Node::new("N1"));
graph.add_edge(Edge::new("N0", "N1", "E"));

let dot_string = graph.to_dot_string().unwrap();

assert_eq!(dot_string,
r#"digraph single_edge {
    "N0"[label="N0"];
    "N1"[label="N1"];
    "N0" -> "N1"[label="E"];
}
"#);
```

If you want add some more attributes, like style, arrow, color,
you could call these methods in a chain, like:

```rust
use dot_graph::{Graph, Kind, Node, Edge, Style};

let mut graph = Graph::new("single_edge", Kind::Digraph);

graph.add_node(Node::new("N0"));
graph.add_node(Node::new("N1"));
graph.add_edge(Edge::new("N0", "N1", "E").style(Style::Bold).color(Some("red")));

assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_edge {
    "N0"[label="N0"];
    "N1"[label="N1"];
    "N0" -> "N1"[label="E"][style="bold"][color="red"];
}
"#);
```

After version 0.2.1, dot_graph support subgraph generation, for example:

```rust
#[test]
fn test_subgraph() {
    let mut graph = Graph::new("di", Kind::Digraph);
    let mut c1 = Subgraph::new("cluster_0").label("");
    c1.add_node(Node::new("N0"));
    c1.add_node(Node::new("N1"));
    let mut c2 = Subgraph::new("cluster_1").label("");
    c2.add_node(Node::new("N2"));
    c2.add_node(Node::new("N3"));
    graph.add_subgraph(c1);
    graph.add_subgraph(c2);
    graph.add_edge(Edge::new("N0", "N1", ""));
    graph.add_edge(Edge::new("N0", "N2", ""));
    graph.add_edge(Edge::new("N1", "N3", ""));
    graph.add_edge(Edge::new("N2", "N3", ""));
    

    assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph di {
subgraph cluster_0 {
    label="";
    "N0"[label="N0"];
    "N1"[label="N1"];
}
subgraph cluster_1 {
    label="";
    "N2"[label="N2"];
    "N3"[label="N3"];
}
"N0" -> "N1"[label=""];
"N0" -> "N2"[label=""];
"N1" -> "N3"[label=""];
"N2" -> "N3"[label=""];
}
"#);
}
```

For more examples, please check the tests.

The library is under active development, we'll include more dot attributes
in the future.

## Roadmap (TODO list)
- support for subgraph

## Contributing
- All sorts of contributing are welcome. create issue and/or PR as you please.
- We belive that TDD(Test-Driven Development) approach is helpful not only in development, but also in communication with each other. So adding more tests might be a good way to report a bug or even suggest a new feature.

## License
dot_graph is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

## Related Project
[rudg](https://github.com/SelamaAshalanore/rudg): Rust UML Diagram Generator. A library for generating UML diagram from Rust source code.
