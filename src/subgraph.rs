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
    color: Option<&'static str>,
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

    pub fn color(&self, color: Option<&'static str>) -> Self {
        let mut subg = self.clone();
        subg.color = color;
        subg
    }
}