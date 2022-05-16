use crate::{
    node::Node,
    style::Style
};

/// `Graph`'s subgraph
#[derive(Clone)]
pub struct Subgraph {
    pub name: String,
    nodes: Vec<Node>,
    label: String,
    style: Style,
    color: Option<String>,
}

impl Subgraph {
    pub fn new(name: &str) -> Self {
        Subgraph { name: String::from(name), nodes: vec![], label: String::new(), style: Style::None, color: None}
    }

    pub fn add_node(&self, node: Node) -> Self {
        let mut subg = self.clone();
        subg.nodes.push(node);
        subg
    }

    pub fn add_nodes(&self, nodes: Vec<Node>) -> Self {
        let mut subg = self.clone();
        subg.nodes.append(&mut nodes.clone());
        subg
    }

    pub fn label(&self, label: &str) -> Self {
        let mut subg = self.clone();
        subg.label = String::from(label);
        subg
    }

    pub fn style(&self, style: Style) -> Self {
        let mut subg = self.clone();
        subg.style = style;
        subg
    }

    pub fn color(&self, color: Option<&str>) -> Self {
        let mut subg = self.clone();
        subg.color = match color {
            Some(c) => Some(String::from(c)),
            None => None
        };
        subg
    }

    pub fn to_dot_string(&self) -> String {
        let mut text = vec!["subgraph ", self.name.as_str(), " {\n    "];

        text.push("label=\"");
        text.push(self.label.as_str());
        text.push("\";\n    ");

        for node in &self.nodes {
            text.push(node.name.as_str());
            text.push(";\n    ");
        }

        text.push("}");
        return text.into_iter().collect();
    }
}