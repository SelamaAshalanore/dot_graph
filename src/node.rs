/// each node is an index in a vector in the graph.
// pub type Node = usize;
use crate::{style::Style, utils::quote_string};

/// `Graph`'s node
#[derive(Clone)]
pub struct Node {
    pub name: String,
    label: Option<String>,
    style: Style,
    color: Option<String>,
    shape: Option<String>,
    url: String,
    attribs: Vec<String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: new_name(name),
            label: None,
            style: Style::None,
            color: None,
            shape: None,
            url: Default::default(),
            attribs: vec![],
        }
    }

    pub fn label(&self, label: &str) -> Self {
        let mut node = self.clone();
        node.label = Some(String::from(label));
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
            None => node.shape = None,
        }
        node
    }

    pub fn color(&self, color: Option<&str>) -> Self {
        let mut node = self.clone();
        node.color = match color {
            Some(c) => Some(String::from(c)),
            None => None,
        };
        node
    }

    pub fn url(&mut self, url: String) -> Self {
        let mut node = self.clone();
        node.url = url;
        node
    }

    pub fn attrib(&self, name: &str, value: &str) -> Self {
        let mut node = self.clone();
        node.attribs.push(format!("{}={}", name, value));
        node
    }

    pub fn to_dot_string(&self) -> String {
        let colorstring: String;

        let escaped_url: String = quote_string(self.url.clone());
        let shape: String;

        let mut text = vec!["\"", self.name.as_str(), "\""];

        let escaped_label: String = if self.label.is_some() {
            quote_string(self.label.clone().unwrap())
        } else {
            quote_string("".to_owned())
        };

        if self.label.is_some() {
            text.push("[label=");
            text.push(escaped_label.as_str());
            text.push("]");
        }

        let binding = self.attribs.join(",");
        if !self.attribs.is_empty() {
            text.push("[");
            text.push(binding.as_str());
            text.push("]");
        }

        if !self.url.is_empty() {
            text.push("[URL=");
            text.push(escaped_url.as_str());
            text.push("]");
        }

        if self.style != Style::None {
            text.push("[style=\"");
            text.push(self.style.as_slice());
            text.push("\"]");
        }

        if let Some(c) = &self.color {
            colorstring = quote_string(c.to_string());
            text.push("[color=");
            text.push(&colorstring);
            text.push("]");
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

/// Check if the node's name is illegal.
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
        Some(c) if is_letter_or_underscore_or_dot(c) => {}
        _ => panic!("The name of the node should start with a letter or underscore or dot"),
    }
    if !chars.all(is_constituent) {
        panic!("The name of the node should only contain letter/number/underscore/dot")
    }
    return String::from(name);

    fn is_letter_or_underscore_or_dot(c: char) -> bool {
        in_range('a', c, 'z') || in_range('A', c, 'Z') || c == '_' || c == '.'
    }
    fn is_constituent(c: char) -> bool {
        is_letter_or_underscore_or_dot(c) || in_range('0', c, '9')
    }
    fn in_range(low: char, c: char, high: char) -> bool {
        low as usize <= c as usize && c as usize <= high as usize
    }
}
