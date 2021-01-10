pub mod graph {
    use std::collections::HashMap;
    use graph_items::{edge::Edge, node::Node};

    #[derive(Eq, PartialEq)]
    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self ,attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|(x,y) | (x.to_string(), y.to_string())).collect();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find( |x| x.name == name)
        }
    }

    pub mod graph_items {

        pub mod node {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self ,attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|(x,y) | (x.to_string(), y.to_string())).collect();
                    self
                }

                pub fn get_attr(&self, attr_name: &str) -> Option<&str> {
                    if let Some(attr) = self.attrs.get(attr_name) { Some(attr.as_str()) } else {None}
                }
            }

        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Clone, Debug)]
            pub struct Edge {
                node1: String,
                node2: String,
                pub attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self ,attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|(x,y) | (x.to_string(), y.to_string())).collect();
                    self
                }
            }
        }

    }
}