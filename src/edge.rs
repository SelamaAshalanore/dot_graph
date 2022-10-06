use crate::{arrow::Arrow, style::Style, utils::quote_string};

/// `Graph`'s edge.
#[derive(Clone)]
pub struct Edge {
    from: String,
    to: String,
    label: String,
    label_url: String,
    url: String,
    style: Style,
    start_arrow: Arrow,
    end_arrow: Arrow,
    color: Option<String>,
}

impl Edge {
    pub fn new(from: &str, to: &str, label: &str) -> Self {
        Edge {
            from: String::from(from),
            to: String::from(to),
            label: String::from(label),
            label_url: Default::default(),
            color: None,
            style: Style::None,
            start_arrow: Arrow::default(),
            end_arrow: Arrow::default(),
            url: Default::default(),
        }
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

    pub fn start_arrow(mut self, arrow: Arrow) -> Self {
        self.start_arrow = arrow;
        self
    }

    pub fn end_arrow(mut self, arrow: Arrow) -> Self {
        self.end_arrow = arrow;
        self
    }

    pub fn label_url(mut self, url: String) -> Self {
        self.label_url = url;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn to_dot_string(&self, edge_symbol: &str) -> String {
        let colorstring: String;
        let escaped_label: &String = &quote_string(self.label.clone());
        let start_arrow_s: String = self.start_arrow.to_dot_string();
        let end_arrow_s: String = self.end_arrow.to_dot_string();
        let escaped_label_url: &String = &quote_string(self.label_url.clone());
        let escaped_url: &String = &quote_string(self.url.clone());

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

        text.push("[label=");
        text.push(escaped_label.as_str());
        text.push("]");

        if !self.label_url.is_empty() {
            text.push("[labelURL=");
            text.push(escaped_label_url.as_str());
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

        let color: Option<String> = match &self.color {
            Some(l) => Some((*l).clone()),
            None => None,
        };
        if let Some(c) = color {
            colorstring = quote_string(c);
            text.push("[color=");
            text.push(&colorstring);
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
