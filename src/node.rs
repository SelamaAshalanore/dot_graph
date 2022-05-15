/// each node is an index in a vector in the graph.
// pub type Node = usize;

use crate::{
    style::Style,
    utils::quote_string,
    render::{RenderOption}
};

#[derive(Clone)]
pub struct Node {
    pub name: String,
    label: String,
    style: Style,
    color: Option<&'static str>,
    shape: Option<String>
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node { name: String::from(name), label: String::from(name), style: Style::None, color: None, shape: None }
    }

    pub fn label(&self, label: &str) -> Self {
        let mut node = self.clone();
        node.label = String::from(label);
        node
    }

    pub fn style(&self, style: Style) -> Self {
        let mut node = self.clone();
        node.style = style;
        node
    }

    pub fn shape(&self, shape: Option<&str>) -> Self {
        let mut node = self.clone();
        match shape {
            Some(s) => node.shape = Some(String::from(s)),
            None => node.shape = None
        }
        node
    }

    pub fn color(&self, color: Option<&'static str>) -> Self {
        let mut node = self.clone();
        node.color = color;
        node
    }

    pub fn node_id(&self) -> &str {
        self.name.as_str()
    }

    pub fn to_dot_string(&self, options: &[RenderOption]) -> String {
        let colorstring: String;

        let escaped: String = quote_string(self.label.clone());
        let shape: String;

        let mut text = vec![self.node_id()];

        if !options.contains(&RenderOption::NoNodeLabels) {
            text.push("[label=");
            text.push(escaped.as_str());
            text.push("]");
        }

        let style = self.style;
        if !options.contains(&RenderOption::NoNodeStyles) && style != Style::None {
            text.push("[style=\"");
            text.push(style.as_slice());
            text.push("\"]");
        }

        let color = self.color;
        if !options.contains(&RenderOption::NoNodeColors) {
            if let Some(c) = color {
                colorstring = quote_string(c.to_string());
                text.push("[color=");
                text.push(&colorstring);
                text.push("]");
            }
        }

        if let Some(s) = self.shape.clone() {
            shape = s;
            text.push("[shape=\"");
            text.push(&shape);
            text.push("\"]");
        }

        text.push(";");
        return text.into_iter().collect();
    }

}