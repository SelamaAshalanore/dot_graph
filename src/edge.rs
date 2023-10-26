use crate::{arrow::Arrow, style::Style, utils::quote_string};

/// `Graph`'s edge.
#[derive(Clone)]
pub struct Edge {
    from: String,
    to: String,
    start_arrow: Arrow,
    end_arrow: Arrow,
    attribs: Vec<String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: String::from(from),
            to: String::from(to),
            start_arrow: Arrow::default(),
            end_arrow: Arrow::default(),
            attribs: vec![],
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.attrib("label", label)
    }

    pub fn style(&mut self, style: Style) -> Self {
        self.attrib("style", style.as_slice())
    }

    pub fn color(&mut self, color: &str) -> Self {
        self.attrib("color", &quote_string(color.to_owned()))
    }

    pub fn attrib(&self, name: &str, value: &str) -> Self {
        let mut edge = self.clone();
        edge.attribs.push(format!("{}={}", name, value));
        edge
    }

    pub fn start_arrow(&mut self, arrow: Arrow) -> Self {
        let mut edge = self.clone();
        edge.start_arrow = arrow;
        edge
    }

    pub fn end_arrow(&mut self, arrow: Arrow) -> Self {
        let mut edge = self.clone();
        edge.end_arrow = arrow;
        edge
    }

    pub fn label_url(&mut self, url: &str) -> Self {
        self.attrib("labelURL", &quote_string(url.to_owned()))
    }

    pub fn url(&mut self, url: &str) -> Self {
        self.attrib("URL", &quote_string(url.to_owned()))
    }

    pub fn to_dot_string(&self, edge_symbol: &str) -> String {
        let start_arrow_s: String = self.start_arrow.to_dot_string();
        let end_arrow_s: String = self.end_arrow.to_dot_string();

        let mut text = vec![
            "\"",
            self.from.as_str(),
            "\" ",
            edge_symbol,
            " ",
            "\"",
            self.to.as_str(),
            "\"",
        ];

        let binding = self.attribs.join(",");
        if !self.attribs.is_empty() {
            text.push("[");
            text.push(binding.as_str());
            text.push("]");
        }

        let mut arrow_text: Vec<String> = vec![];
        let mut arrow_str: String = String::new();
        if !self.start_arrow.is_default() || !self.end_arrow.is_default() {
            if !self.end_arrow.is_default() {
                arrow_text.push(
                    vec!["arrowhead=\"", &end_arrow_s, "\""]
                        .into_iter()
                        .collect(),
                );
            }
            if !self.start_arrow.is_default() {
                arrow_text.push(
                    vec!["arrowtail=\"", &start_arrow_s, "\""]
                        .into_iter()
                        .collect(),
                );
            }
            if !self.start_arrow.is_default() && !self.end_arrow.is_default() {
                arrow_text.push(String::from("dir=\"both\""));
            }
        }
        if arrow_text.len() > 0 {
            arrow_str.push_str(&arrow_text.join(" "));
            arrow_str.insert(0, '[');
            arrow_str.push_str("]");
            text.push(arrow_str.as_str());
        }
        text.push(";");
        return text.into_iter().collect();
    }
}
