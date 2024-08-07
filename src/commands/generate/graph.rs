use crate::{cli, Error, Result};

pub fn generate_graph<R: rand::Rng>(
    args: &cli::generate::GenerateConfig,
    rng: &mut R,
) -> Result<creo_lib::graph::ColoredGraph> {
    let params = creo_lib::graph::algorithms::random::GNMParameters {
        node_count: args.number_of_endpoints,
        edge_count: args.number_of_service_calls,
        maximum_degree: Some(args.number_of_services),
    };
    let graph = creo_lib::graph::algorithms::random::random_gnm_graph(&params, rng);
    let graph = color_graph(graph, args.number_of_services)?;
    Ok(graph)
}

fn color_graph(
    graph: creo_lib::graph::DiGraph,
    color_count: usize,
) -> Result<creo_lib::graph::ColoredGraph> {
    let coloring =
        creo_lib::graph::algorithms::coloring::equitable_coloring(&graph, color_count)
            .map_err(|err| Error::with_log("could not find a valid coloring".to_string(), err))?;

    Ok(creo_lib::graph::ColoredGraphBuilder::new(graph, coloring, color_count).build())
}

pub fn generate_graph_with_edges(
    args: &cli::generate::GenerateConfig,
) -> Result<creo_lib::graph::ColoredGraph> {
    let edges: Vec<_> = args
        .service_call_list
        .iter()
        .cloned()
        .map(creo_lib::graph::Edge::from)
        .collect();
    let graph = creo_lib::graph::DiGraphBuilder::with_node_and_edge_count(
        args.number_of_endpoints,
        args.service_call_list.len(),
    )
    .add_nodes(args.number_of_endpoints)
    .add_edges(&edges)
    .build();
    let graph = color_graph(graph, args.number_of_services)?;
    Ok(graph)
}
