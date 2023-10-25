use crate::{edge::Edge, node::Node, subgraph::Subgraph, utils::quote_string};
use std::io;
use std::io::prelude::*;

/// Entry point of this library, use `to_dot_string` to get the string output.
#[derive(Clone)]
pub struct Graph {
    name: String,
    kind: Kind,
    url: String,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    subgraph: Vec<Subgraph>,
    attribs: Vec<String>,
}

impl Graph {
    pub fn new(name: &str, kind: Kind) -> Graph {
        Graph {
            name: String::from(name),
            kind: kind,
            nodes: vec![],
            edges: vec![],
            subgraph: vec![],
            url: Default::default(),
            attribs: vec![],
        }
    }

    pub fn add_node(&mut self, node: Node) -> () {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) -> () {
        self.edges.push(edge);
    }

    pub fn add_subgraph(&mut self, subgraph: Subgraph) -> () {
        self.subgraph.push(subgraph.edgeop(self.kind.edgeop()))
    }

    pub fn url(&mut self, url: String) -> Self {
        let mut graph = self.clone();
        graph.url = url;
        graph
    }

    pub fn to_dot_string(&self) -> io::Result<String> {
        let mut writer = Vec::new();
        self.render_opts(&mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

    pub fn attrib(&self, name: &str, value: &str) -> Self {
        let mut graph = self.clone();
        graph.attribs.push(format!("{}={}", name, value));
        graph
    }

    /// Renders graph `g` into the writer `w` in DOT syntax.
    /// (Main entry point for the library.)
    fn render_opts<'a, W: Write>(&self, w: &mut W) -> io::Result<()> {
        fn writeln<W: Write>(w: &mut W, arg: &[&str]) -> io::Result<()> {
            for &s in arg {
                w.write_all(s.as_bytes())?;
            }
            write!(w, "\n")
        }

        fn indent<W: Write>(w: &mut W) -> io::Result<()> {
            w.write_all(b"    ")
        }

        writeln(w, &[self.kind.keyword(), " ", self.name.as_str(), " {"])?;

        for attrib in &self.attribs {
            indent(w)?;
            writeln(w, &[&attrib])?;
        }

        if !self.url.is_empty() {
            indent(w)?;
            writeln(w, &["URL=", quote_string(self.url.clone()).as_str()])?;
        }

        for n in self.subgraph.iter() {
            indent(w)?;
            let mut text: Vec<&str> = vec![];
            let subgraph_dot_string: String = n.to_dot_string();
            text.push(&subgraph_dot_string.as_str());
            writeln(w, &text)?;
        }

        for n in self.nodes.iter() {
            indent(w)?;
            let mut text: Vec<&str> = vec![];
            let node_dot_string: String = n.to_dot_string();
            text.push(&node_dot_string.as_str());
            writeln(w, &text)?;
        }

        let edge_symbol = self.kind.edgeop();
        for e in self.edges.iter() {
            indent(w)?;
            let mut text: Vec<&str> = vec![];
            let edge_dot_string: String = e.to_dot_string(edge_symbol);
            text.push(&edge_dot_string.as_str());
            writeln(w, &text)?;
        }

        writeln(w, &["}"])
    }
}

/// Graph kind determines if `digraph` or `graph` is used as keyword
/// for the graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Digraph,
    Graph,
}

impl Kind {
    /// The keyword to use to introduce the graph.
    /// Determines which edge syntax must be used, and default style.
    pub fn keyword(&self) -> &'static str {
        match *self {
            Kind::Digraph => "digraph",
            Kind::Graph => "graph",
        }
    }

    /// The edgeop syntax to use for this graph kind.
    pub fn edgeop(&self) -> &'static str {
        match *self {
            Kind::Digraph => "->",
            Kind::Graph => "--",
        }
    }
}
