// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A library for generating Graphviz DOT language files.
//! 
//! The very basic three parts of a DOT file and in this library is `Graph`,
//! `Node` and `Edge`. `Graph` is the entry point of the library. You could
//! generate an empty graph .dot file by simply use:
//! 
//! ```rust
//! use dot_graph::{Graph, Kind};
//! 
//! let graph = Graph::new("empty_graph", Kind::Digraph);
//! 
//! let dot_string = graph.to_dot_string().unwrap();
//! 
//! assert_eq!(dot_string,
//!    r#"digraph empty_graph {
//! }
//!"#);
//! ```
//!
//! In order to add some basic nodes and edges:
//! 
//! ```rust
//! use dot_graph::{Graph, Kind, Node, Edge};
//! 
//! let mut graph = Graph::new("single_edge", Kind::Digraph);
//! 
//! graph.add_node(Node::new("N0"));
//! graph.add_node(Node::new("N1"));
//! graph.add_edge(Edge::new("N0", "N1", "E"));
//! 
//! let dot_string = graph.to_dot_string().unwrap();
//! 
//! assert_eq!(dot_string,
//! r#"digraph single_edge {
//!     N0[label="N0"];
//!     N1[label="N1"];
//!     N0 -> N1[label="E"];
//! }
//! "#);
//! ```
//! 
//! If you want add some more attributes, like style, arrow, color,
//! you could call these methods in a chain, like:
//! 
//! ```rust
//! use dot_graph::{Graph, Kind, Node, Edge, Style};
//! 
//! let mut graph = Graph::new("single_edge", Kind::Digraph);
//! 
//! graph.add_node(Node::new("N0"));
//! graph.add_node(Node::new("N1"));
//! graph.add_edge(Edge::new("N0", "N1", "E").style(Style::Bold).color(Some("red")));
//! 
//! assert_eq!(graph.to_dot_string().unwrap(),
//! r#"digraph single_edge {
//!     N0[label="N0"];
//!     N1[label="N1"];
//!     N0 -> N1[label="E"][style="bold"][color="red"];
//! }
//! "#);
//! ```
//! 
//! For more examples, please check the tests.
//! 
//! The library is under active development, we'll include more dot attributes
//! in the future.
//! 
//! # References
//!
//! * [Graphviz](http://graphviz.org/)
//!
//! * [DOT language](http://graphviz.org/doc/info/lang.html)

mod style;
mod arrow;
mod node;
mod edge;
mod graph;
mod utils;
mod subgraph;

pub use style::Style;
pub use arrow::{Arrow, ArrowShape, Side, Fill};
pub use node::{Node};
pub use edge::{Edge};
pub use graph::{Graph, Kind};

