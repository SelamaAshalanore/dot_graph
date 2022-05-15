
#[cfg(test)]
mod tests {
    use dot_graph::{LabelledGraph, edge, edge_with_arrows, Graph, Kind, Node, Edge};
    use dot_graph::{Id, render, Style, DefaultStyleGraph, id_name};
    use dot_graph::{Arrow, ArrowShape, Side};
    use std::io;
    use std::io::prelude::*;

    fn test_input(g: LabelledGraph) -> io::Result<String> {
        let mut writer = Vec::new();
        render(&g, &mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

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
        let mut node = Node::new("N0");
        node.set_style(Style::Dashed);
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
        let mut e = Edge::new("N0", "N1", "E");
        e.set_style(Style::Bold);
        e.set_color(Some("red"));
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
        let styles = Some(vec![Style::None, Style::Dotted]);
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge(id_name(&0).as_slice(), id_name(&1).as_slice(), "A-1", Style::None, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#);
    }

    #[test]
    fn single_cyclic_node() {
        let r = test_input(LabelledGraph::new("single_cyclic_node",
                                              vec![None],
                                              vec![edge(id_name(&0).as_slice(), id_name(&0).as_slice(), "E", Style::None, None)],
                                              None));
        assert_eq!(r.unwrap(),
r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#);
    }

    #[test]
    fn hasse_diagram() {
        let r = test_input(LabelledGraph::new("hasse_diagram",
                                              vec![Some("{x,y}"), Some("{x}"), Some("{y}"), Some("{}")],
                                              vec![edge(id_name(&0).as_slice(), id_name(&1).as_slice(), "", Style::None, Some("green")),
                                                   edge(id_name(&0).as_slice(), id_name(&2).as_slice(), "", Style::None, Some("blue")),
                                                   edge(id_name(&1).as_slice(), id_name(&3).as_slice(), "", Style::None, Some("red")),
                                                   edge(id_name(&2).as_slice(), id_name(&3).as_slice(), "", Style::None, Some("black"))],
                                              None));
        assert_eq!(r.unwrap(),
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
        let mut writer = Vec::new();

        let g = LabelledGraph::new("syntax_tree",
                                              vec!(Some(
r#"if test {
\l    branch1
\l} else {
\l    branch2
\l}
\lafterward
\l"#),
            Some("branch1"),
            Some("branch2"),
            Some("afterward")),
                                              vec![edge(id_name(&0).as_slice(), id_name(&1).as_slice(), "then", Style::None, None),
                                                   edge(id_name(&0).as_slice(), id_name(&2).as_slice(), "else", Style::None, None),
                                                   edge(id_name(&1).as_slice(), id_name(&3).as_slice(), ";", Style::None, None),
                                                   edge(id_name(&2).as_slice(), id_name(&3).as_slice(), ";", Style::None, None)],
                                                None);

        render(&g, &mut writer).unwrap();
        let mut r = String::new();
        Read::read_to_string(&mut &*writer, &mut r).unwrap();

        assert_eq!(r,
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
    fn simple_id_construction() {
        let id1 = Id::new("hello");
        match id1 {
            Ok(_) => {}
            Err(..) => panic!("'hello' is not a valid value for id anymore"),
        }
    }

    #[test]
    fn test_some_arrow() {
        let styles = Some(vec![Style::None, Style::Dotted]);
        let start  = Arrow::default();
        let end    = Arrow::from_arrow(ArrowShape::crow());
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge_with_arrows(id_name(&0).as_slice(), id_name(&1).as_slice(), "A-1", Style::None, start, end, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="crow"];
}
"#);
    }

    #[test]
    fn test_some_arrows() {
        let styles = Some(vec![Style::None, Style::Dotted]);
        let start  = Arrow::from_arrow(ArrowShape::tee());
        let end    = Arrow::from_arrow(ArrowShape::Crow(Side::Left));
        let result = test_input(LabelledGraph::new("test_some_labelled",
                                                   vec![Some("A"), None],
                                                   vec![edge_with_arrows(id_name(&0).as_slice(), id_name(&1).as_slice(), "A-1", Style::None, start, end, None)],
                                                   styles));
        assert_eq!(result.unwrap(),
r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="lcrow" arrowtail="tee" dir="both"];
}
"#);
    }

    #[test]
    fn invisible() {
        let r = test_input(LabelledGraph::new("single_cyclic_node",
                                              vec![None],
                                              vec![edge(id_name(&0).as_slice(), id_name(&0).as_slice(), "E", Style::Invisible, None)],
                                              Some(vec![Style::Invisible])));
        assert_eq!(r.unwrap(),
                   r#"digraph single_cyclic_node {
    N0[label="N0"][style="invis"];
    N0 -> N0[label="E"][style="invis"];
}
"#);
    }

    #[test]
    fn badly_formatted_id() {
        let id2 = Id::new("Weird { struct : ure } !!!");
        match id2 {
            Ok(_) => panic!("graphviz id suddenly allows spaces, brackets and stuff"),
            Err(..) => {}
        }
    }

    fn test_input_default(g: DefaultStyleGraph) -> io::Result<String> {
        let mut writer = Vec::new();
        render(&g, &mut writer).unwrap();
        let mut s = String::new();
        Read::read_to_string(&mut &*writer, &mut s)?;
        Ok(s)
    }

    #[test]
    fn default_style_graph() {
        let r = test_input_default(
            DefaultStyleGraph::new("g", 4,
                                   vec![(0, 1), (0, 2), (1, 3), (2, 3)],
                                   Kind::Graph));
        assert_eq!(r.unwrap(),
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
    fn default_style_digraph() {
        let r = test_input_default(
            DefaultStyleGraph::new("di", 4,
                                   vec![(0, 1), (0, 2), (1, 3), (2, 3)],
                                   Kind::Digraph));
        assert_eq!(r.unwrap(),
r#"digraph di {
    N0[label="N0"];
    N1[label="N1"];
    N2[label="N2"];
    N3[label="N3"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
    N1 -> N3[label=""];
    N2 -> N3[label=""];
}
"#);
    }
}