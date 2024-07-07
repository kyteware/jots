use iced::{widget::{button, row, text}, Element};
use petgraph::graph::DiGraph;

use crate::goal_tree::{goal_tree, GraphNode};

pub struct App {
    graph: DiGraph<GraphNode, ()>
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Inc
}

impl App {
    pub fn new() -> Self {
        let mut graph = DiGraph::new();
        let a = graph.add_node(GraphNode {
            name: "do something".to_string(),
            complete: true,
            coords: (100., 100.)
        });
        let b = graph.add_node(GraphNode {
            name: "done".to_string(),
            complete: false,
            coords: (300., 100.)
        });
        graph.add_edge(a, b, ());
        App { graph }
    }

    pub fn update(&mut self, msg: Message) {
        //
    }

    pub fn view(&self) -> Element<Message> {
        row!(goal_tree(&self.graph)).into()
    }
}
