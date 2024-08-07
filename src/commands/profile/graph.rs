pub fn generate_graph(node_count: usize) -> creo_lib::graph::ColoredGraph {
    let graph = creo_lib::graph::DiGraphBuilder::with_node_count(node_count)
        .add_nodes(node_count)
        .build();
    color_graph(graph)
}

fn color_graph(graph: creo_lib::graph::DiGraph) -> creo_lib::graph::ColoredGraph {
    let coloring: Vec<creo_lib::graph::algorithms::coloring::ColorIndex> =
        Vec::from_iter(graph.iter_nodes().map(|index| index.0.into()));
    let color_count = graph.node_count();
    creo_lib::graph::ColoredGraphBuilder::new(graph, coloring, color_count).build()
}
