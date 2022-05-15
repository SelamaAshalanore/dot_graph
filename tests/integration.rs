
#[cfg(test)]
mod tests {
    use dot_graph::{Graph, Kind, Node, Edge};
    use dot_graph::{Style};
    use dot_graph::{Arrow, ArrowShape, Side};

    // All of the tests use raw-strings as the format for the expected outputs,
    // so that you can cut-and-paste the content into a .dot file yourself to
    // see what the graphviz visualizer would produce.

    #[test]
    fn empty_graph() {
        let graph = Graph::new("empty_graph", Kind::Digraph);
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph empty_graph {
}
"#);
    }

    #[test]
    fn single_node() {
        let mut graph = Graph::new("single_node", Kind::Digraph);
        let node = Node::new("N0");
        graph.add_node(node);
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_node {
    N0[label="N0"];
}
"#);
    }

    #[test]
    fn single_node_with_style() {
        let mut graph = Graph::new("single_node", Kind::Digraph);
        let node = Node::new("N0").style(Style::Dashed);
        graph.add_node(node);
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#);
    }

    #[test]
    fn single_edge() {
        let mut graph = Graph::new("single_edge", Kind::Digraph);
        graph.add_node(Node::new("N0"));
        graph.add_node(Node::new("N1"));
        graph.add_edge(Edge::new("N0", "N1", "E"));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"];
}
"#);
    }

    #[test]
    fn single_edge_with_style() {
        let mut graph = Graph::new("single_edge", Kind::Digraph);
        graph.add_node(Node::new("N0"));
        graph.add_node(Node::new("N1"));
        let e = Edge::new("N0", "N1", "E").style(Style::Bold).color(Some("red"));
        graph.add_edge(e);
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"][style="bold"][color="red"];
}
"#);
    }

    #[test]
    fn test_some_labelled() {
        let mut graph = Graph::new("test_some_labelled", Kind::Digraph);
        graph.add_node(Node::new("N0").label("A"));
        graph.add_node(Node::new("N1").style(Style::Dotted));
        graph.add_edge(Edge::new("N0", "N1", "A-1"));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#);
    }

    #[test]
    fn single_cyclic_node() {
        let mut graph = Graph::new("single_cyclic_node", Kind::Digraph);
        graph.add_node(Node::new("N0"));
        graph.add_edge(Edge::new("N0", "N0", "E"));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#);
    }

    #[test]
    fn hasse_diagram() {
        let mut graph = Graph::new("hasse_diagram", Kind::Digraph);
        graph.add_node(Node::new("N0").label("{x,y}"));
        graph.add_node(Node::new("N1").label("{x}"));
        graph.add_node(Node::new("N2").label("{y}"));
        graph.add_node(Node::new("N3").label("{}"));
        graph.add_edge(Edge::new("N0", "N1", "").color(Some("green")));
        graph.add_edge(Edge::new("N0", "N2", "").color(Some("blue")));
        graph.add_edge(Edge::new("N1", "N3", "").color(Some("red")));
        graph.add_edge(Edge::new("N2", "N3", "").color(Some("black")));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph hasse_diagram {
    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""][color="green"];
    N0 -> N2[label=""][color="blue"];
    N1 -> N3[label=""][color="red"];
    N2 -> N3[label=""][color="black"];
}
"#);
    }

    #[test]
    fn left_aligned_text() {
        let mut graph = Graph::new("syntax_tree", Kind::Digraph);
        let node_label = 
r#"if test {
\l    branch1
\l} else {
\l    branch2
\l}
\lafterward
\l"#;
        graph.add_node(Node::new("N0").label(node_label));
        graph.add_node(Node::new("N1").label("branch1"));
        graph.add_node(Node::new("N2").label("branch2"));
        graph.add_node(Node::new("N3").label("afterward"));
        graph.add_edge(Edge::new("N0", "N1", "then"));
        graph.add_edge(Edge::new("N0", "N2", "else"));
        graph.add_edge(Edge::new("N1", "N3", ";"));
        graph.add_edge(Edge::new("N2", "N3", ";"));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph syntax_tree {
    N0[label="if test {
\l    branch1
\l} else {
\l    branch2
\l}
\lafterward
\l"];
    N1[label="branch1"];
    N2[label="branch2"];
    N3[label="afterward"];
    N0 -> N1[label="then"];
    N0 -> N2[label="else"];
    N1 -> N3[label=";"];
    N2 -> N3[label=";"];
}
"#);
    }

    #[test]
    fn test_some_arrow() {
        let mut graph = Graph::new("test_some_labelled", Kind::Digraph);
        graph.add_node(Node::new("N0").label("A"));
        graph.add_node(Node::new("N1").style(Style::Dotted));
        graph.add_edge(Edge::new("N0", "N1", "A-1").end_arrow(Arrow::from_arrow(ArrowShape::crow())));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="crow"];
}
"#);
    }

    #[test]
    fn test_some_arrows() {
        let mut graph = Graph::new("test_some_labelled", Kind::Digraph);
        graph.add_node(Node::new("N0").label("A"));
        graph.add_node(Node::new("N1").style(Style::Dotted));
        graph.add_edge(Edge::new("N0", "N1", "A-1").end_arrow(Arrow::from_arrow(ArrowShape::Crow(Side::Left))).start_arrow(Arrow::from_arrow(ArrowShape::tee())));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="lcrow" arrowtail="tee" dir="both"];
}
"#);
    }

    #[test]
    fn invisible() {
        let mut graph = Graph::new("single_cyclic_node", Kind::Digraph);
        graph.add_node(Node::new("N0").style(Style::Invisible));
        graph.add_edge(Edge::new("N0", "N0", "E").style(Style::Invisible));
        assert_eq!(graph.to_dot_string().unwrap(),
                   r#"digraph single_cyclic_node {
    N0[label="N0"][style="invis"];
    N0 -> N0[label="E"][style="invis"];
}
"#);
    }

    #[test]
    fn default_style_graph() {
        let mut graph = Graph::new("g", Kind::Graph);
        graph.add_node(Node::new("N0"));
        graph.add_node(Node::new("N1"));
        graph.add_node(Node::new("N2"));
        graph.add_node(Node::new("N3"));
        graph.add_edge(Edge::new("N0", "N1", ""));
        graph.add_edge(Edge::new("N0", "N2", ""));
        graph.add_edge(Edge::new("N1", "N3", ""));
        graph.add_edge(Edge::new("N2", "N3", ""));
        assert_eq!(graph.to_dot_string().unwrap(),
r#"graph g {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -- N1[label=""];
    N0 -- N2[label=""];
    N1 -- N3[label=""];
    N2 -- N3[label=""];
}
"#);
    }

    #[test]
    #[should_panic]
    fn badly_formatted_id() {
        let mut graph = Graph::new("g", Kind::Graph);
        graph.add_node(Node::new("Weird { struct : ure } !!!"));
        let result = graph.to_dot_string();
        result.unwrap();
    }

    #[test]
    #[should_panic]
    fn edge_has_unrecognized_nodes() {
        let mut graph = Graph::new("g", Kind::Graph);
        graph.add_edge(Edge::new("N0", "N1", "test"));
        let result = graph.to_dot_string();
        result.unwrap();
    }
}