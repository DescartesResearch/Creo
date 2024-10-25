use crate::{cli, Error, Result};

pub fn generate_graph<R: rand::Rng>(
    args: &cli::generate::Config,
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
    args: &cli::generate::Config,
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand_seeder as rng;

    #[test]
    fn test_generate_graph() {
        let args = cli::generate::Config {
            service_call_list: Vec::default(),
            number_of_endpoints: 4,
            selected_languages: Vec::default(),
            app_name: "Testapp".into(),
            number_of_service_calls: 3,
            number_of_services: 3,
            service_types: creo_lib::ServiceTypeVec(Vec::default()),
            seed: "testseed".into(),
            start_port: 30100,
        };
        let mut rng: rng::SipRng = rng::Seeder::from(&args.seed).make_rng();

        let graph = generate_graph(&args, &mut rng).expect("successful graph generation");
        assert_eq!(graph.graph.node_count(), 4, "should have 4 vertices");
        let edges: Vec<_> = graph.graph.iter_edges().collect();
        assert_eq!(edges.len(), 3, "should have 3 edges");
        assert_eq!(graph.color_count(), 3, "should have 3 colors");
    }

    #[test]
    fn test_generate_graph_with_edges_properties() {
        let e0 = (0, 1);
        let e1 = (1, 2);
        let e2 = (0, 2);

        let args = cli::generate::Config {
            service_call_list: Vec::from([e0, e1, e2]),
            number_of_endpoints: 4,
            selected_languages: Vec::default(),
            app_name: "Testapp".into(),
            number_of_service_calls: 0,
            number_of_services: 3,
            service_types: creo_lib::ServiceTypeVec(Vec::default()),
            seed: "testseed".into(),
            start_port: 30100,
        };

        let graph = generate_graph_with_edges(&args).expect("successful graph generation");

        assert_eq!(graph.graph.node_count(), 4, "should have 4 vertices");
        let mut edges: Vec<_> = graph.graph.iter_edges().collect();
        assert_eq!(edges.len(), 3, "should have 3 edges");
        assert_eq!(
            &edges[..].sort(),
            &[
                creo_lib::graph::Edge::from(e0),
                creo_lib::graph::Edge::from(e1),
                creo_lib::graph::Edge::from(e2)
            ]
            .sort(),
            "unexpected edges in graph"
        );
        assert_eq!(graph.color_count(), 3, "should have 2 colors");
    }
}
