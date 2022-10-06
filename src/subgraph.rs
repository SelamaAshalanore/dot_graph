use crate::{node::Node, style::Style, utils::quote_string, Edge, Kind};

/// `Graph`'s subgraph
#[derive(Clone)]
pub struct Subgraph {
    pub name: String,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    label: String,
    style: Style,
    color: Option<String>,
    edgeop: String,
    url: String,
}

impl Subgraph {
    pub fn new(name: &str) -> Self {
        Subgraph {
            name: new_name(name),
            nodes: vec![],
            edges: vec![],
            label: String::new(),
            style: Style::None,
            color: None,
            edgeop: String::from(Kind::Digraph.edgeop()),
            url: Default::default(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> () {
        self.nodes.push(node);
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) -> () {
        self.nodes.append(&mut nodes.clone());
    }

    pub fn add_edge(&mut self, edge: Edge) -> () {
        self.edges.push(edge);
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = String::from(label);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn color(mut self, color: Option<&str>) -> Self {
        self.color = match color {
            Some(c) => Some(String::from(c)),
            None => None,
        };
        self
    }

    pub fn edgeop(mut self, edgeop: &str) -> Self {
        self.edgeop = String::from(edgeop);
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn to_dot_string(&self) -> String {
        let mut text = vec!["subgraph ", self.name.as_str(), " {\n        "];

        let escaped_url: String;
        if !self.url.is_empty() {
            escaped_url = quote_string(self.url.clone());
            text.push("URL=");
            text.push(escaped_url.as_str());
            text.push(";\n        ");
        }

        text.push("label=\"");
        text.push(self.label.as_str());
        text.push("\";\n    ");

        if self.style != Style::None {
            text.push("    style=\"");
            text.push(self.style.as_slice());
            text.push("\";\n    ");
        }

        let colorstring: String;
        if let Some(c) = &self.color {
            colorstring = quote_string(c.to_string());
            text.push("    color=");
            text.push(&colorstring);
            text.push(";\n    ");
        }

        let subgraph_node_names = self
            .nodes
            .iter()
            .map(|n| n.to_dot_string())
            .collect::<Vec<String>>()
            .join("\n        ");
        // in case push extra change line
        if self.nodes.len() > 0 {
            text.push("    ");
            text.push(&subgraph_node_names);
            text.push("\n    ");
        }

        let edge_symbol = &self.edgeop;
        let subgraph_edge_strs = self
            .edges
            .iter()
            .map(|e| e.to_dot_string(&edge_symbol))
            .collect::<Vec<String>>()
            .join("\n        ");
        // in case push extra change line
        if self.edges.len() > 0 {
            text.push("    ");
            text.push(&subgraph_edge_strs);
            text.push("\n    ");
        }

        text.push("}");

        return text.into_iter().collect();
    }
}

/// Check if the subgraph's name is illegal.
///
/// The caller must ensure that the input conforms to an
/// identifier format: it must be a non-empty string made up of
/// alphanumeric or underscore characters, not beginning with a
/// digit (i.e. the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`).
///
/// (Note: this format is a strict subset of the `ID` format
/// defined by the DOT language.  This function may change in the
/// future to accept a broader subset, or the entirety, of DOT's
/// `ID` format.)
///
/// Passing an invalid string (containing spaces, brackets,
/// quotes, ...) will cause panic.
fn new_name(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if is_letter_or_underscore(c) => {}
        _ => panic!("The name of the node should start with a letter or underscore or dot"),
    }
    if !chars.all(is_constituent) {
        panic!("The name of the node should only contain letter/number/underscore/dot")
    }

    if !name.starts_with("cluster_") {
        panic!("The name of the subgraph should start with \"cluster_\"")
    }
    return String::from(name);

    fn is_letter_or_underscore(c: char) -> bool {
        in_range('a', c, 'z') || in_range('A', c, 'Z') || c == '_'
    }
    fn is_constituent(c: char) -> bool {
        is_letter_or_underscore(c) || in_range('0', c, '9')
    }
    fn in_range(low: char, c: char, high: char) -> bool {
        low as usize <= c as usize && c as usize <= high as usize
    }
}
