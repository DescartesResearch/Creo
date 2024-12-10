use std::collections::HashMap;

use crate::{Error, Result};

pub struct AutoPilotParameters {
    pub vertices: usize,
    pub edges: usize,
    pub colors: usize,
}

pub fn auto_pilot<R: rand::Rng>(
    params: AutoPilotParameters,
    rng: &mut R,
) -> Result<creo_lib::graph::ColoredGraph> {
    let gnm_params = creo_lib::graph::algorithms::random::GNMParameters {
        node_count: params.vertices,
        edge_count: params.edges,
        maximum_degree: Some(params.colors),
    };
    let graph = creo_lib::graph::algorithms::random::random_gnm_graph(&gnm_params, rng);
    let graph = color_graph(graph, params.colors)?;
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

pub fn manual(params: ManualParameters) -> Result<creo_lib::graph::ColoredGraph> {
    let v_map = params
        .vertices
        .iter()
        .map(VertexDefinition::as_key)
        .enumerate()
        .map(rev2)
        .collect::<HashMap<_, _>>();
    let mut edges = Vec::with_capacity(params.edges.len());
    for edge in params.edges {
        let source = *v_map
            .get(edge.source.as_key())
            .ok_or_else(|| Error::new(format!("invalid edge source {}", edge.source.as_key())))?;
        let target = *v_map
            .get(edge.target.as_key())
            .ok_or_else(|| Error::new(format!("invalid edge target {}", edge.target.as_key())))?;
        edges.push(creo_lib::graph::Edge::from((source, target)));
    }
    let graph = creo_lib::graph::DiGraphBuilder::with_node_and_edge_count(
        params.vertices.len(),
        params.edges.len(),
    )
    .add_nodes(params.vertices.len())
    .add_edges(&edges)
    .build();

    // TODO: Find way to actually return the cycle
    if !graph.is_acyclic() {
        return Err(Error::new("detected a cycle in the specified graph".into()));
    };

    let mut coloring = Vec::with_capacity(params.vertices.len());
    for vertex in params.vertices {
        let color = *params
            .services
            .get(&vertex.microservice)
            .ok_or_else(|| Error::new(format!("invalid service name {}", vertex.microservice)))?;
        coloring.push(creo_lib::graph::algorithms::coloring::ColorIndex::from(
            color,
        ));
    }

    Ok(creo_lib::graph::ColoredGraphBuilder::new(graph, coloring, params.services.len()).build())
}

pub struct VertexDefinition<'a> {
    microservice: &'a str,
    // endpoint: &'a str,
    key: String,
}

impl<'a> VertexDefinition<'a> {
    pub fn new(microservice: &'a str, endpoint: &'a str) -> Self {
        Self {
            microservice,
            key: format!("{}.{}", microservice, endpoint),
        }
    }
    fn as_key(&self) -> &str {
        &self.key
    }
}

pub struct EdgeDefinition<'a> {
    pub source: VertexDefinition<'a>,
    pub target: VertexDefinition<'a>,
}

pub struct ManualParameters<'a, 'b> {
    pub vertices: &'b [VertexDefinition<'a>],
    pub edges: &'b [EdgeDefinition<'a>],
    pub services: &'b HashMap<&'a str, usize>,
}

/// Reverses the order of elements in a 2-tuple.
fn rev2<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_auto_pilot() {
    //     let args = config::graph::AutoPilotConfig {
    //         number_of_endpoints: 4,
    //         number_of_inter_service_calls: 3,
    //         service_config: config::service::AutoPilotConfig {
    //             number_of_services: 3,
    //         },
    //     };
    //     let mut rng = rand::thread_rng();
    //
    //     let graph = auto_pilot(&args, &mut rng).expect("successful graph generation");
    //     assert_eq!(graph.graph.node_count(), 4, "should have 4 vertices");
    //     let edges: Vec<_> = graph.graph.iter_edges().collect();
    //     assert_eq!(edges.len(), 3, "should have 3 edges");
    //     assert_eq!(graph.color_count(), 3, "should have 3 colors");
    //     let coloring: Vec<_> = graph
    //         .iter_nodes_with_colors()
    //         .map(|(_, color)| color)
    //         .collect();
    //     creo_lib::graph::algorithms::coloring::is_coloring(graph.graph.iter_edges(), &coloring)
    //         .unwrap();
    //     assert!(graph.graph.is_acyclic());
    // }
    //
    // #[test]
    // fn test_manual_with_auto_pilot_coloring() {
    //     let e0 = (0, 1);
    //     let e1 = (1, 2);
    //     let e2 = (0, 2);
    //
    //     let args = config::graph::HypridConfig {
    //         inter_service_calls_list: Vec::from([e0, e1, e2]),
    //         number_of_endpoints: 4,
    //         service_mode: config::service::Mode::AutoPilot(config::service::AutoPilotConfig {
    //             number_of_services: 3,
    //         }),
    //     };
    //
    //     let graph = manual(&args).expect("successful graph generation");
    //
    //     assert_eq!(graph.graph.node_count(), 4, "should have 4 vertices");
    //     let mut edges: Vec<_> = graph.graph.iter_edges().collect();
    //     assert_eq!(edges.len(), 3, "should have 3 edges");
    //     assert_eq!(
    //         &edges[..].sort(),
    //         &[
    //             creo_lib::graph::Edge::from(e0),
    //             creo_lib::graph::Edge::from(e1),
    //             creo_lib::graph::Edge::from(e2)
    //         ]
    //         .sort(),
    //         "unexpected edges in graph"
    //     );
    //     assert_eq!(graph.color_count(), 3, "should have 2 colors");
    //     let coloring: Vec<_> = graph
    //         .iter_nodes_with_colors()
    //         .map(|(_, color)| color)
    //         .collect();
    //     creo_lib::graph::algorithms::coloring::is_coloring(graph.graph.iter_edges(), &coloring)
    //         .unwrap();
    //     assert!(graph.graph.is_acyclic());
    // }
}
