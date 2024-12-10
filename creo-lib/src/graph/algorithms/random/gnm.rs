use crate::graph;
use rand::distributions::{self as dist, Distribution};

/// [`GNMParameters`] contains parameters for generating a random graph with `n` nodes and `m`
/// edges.
pub struct GNMParameters {
    /// The number of nodes to generate.
    pub node_count: usize,
    /// The number of edges to generate.
    pub edge_count: usize,
    /// The optional maximum allowed degree of each node
    pub maximum_degree: Option<usize>,
}

/// Generate a random graph with the given number of nodes and edges.
///
/// Optionally restrict the maximum degree in the graph.
/// The edges are uniformly drawn at random from the set of all possible edges, until the graph
/// contains the specified number of edges.
pub fn random_gnm_graph<R: rand::Rng>(params: &GNMParameters, rng: &mut R) -> graph::DiGraph {
    loop {
        let mut builder =
            graph::DiGraphBuilder::with_node_and_edge_count(params.node_count, params.edge_count)
                .add_nodes(params.node_count);

        let distribution = uniform_node_distribution(params.node_count);
        while builder.edge_count() < params.edge_count {
            let (source, target) = select_random_edge(distribution, rng);
            builder = builder.add_edge((source, target).into());
        }

        let graph = builder.build();
        if !graph.is_acyclic() {
            continue;
        }
        match params.maximum_degree {
            None => break graph,
            Some(maximum_degree) => {
                if graph.maximum_degree() < maximum_degree {
                    break graph;
                }
            }
        }
    }
}

/// Create a Uniform distribution for the range of `0` and the given number of nodes.
///
/// This distribution can be used for randomly sampling NodeIDs for a graph with NodeIDs starting
/// from `0` without any gaps.
///
/// # Arguments
///
/// * `node_count` - the number of nodes
fn uniform_node_distribution(node_count: usize) -> dist::Uniform<usize> {
    dist::Uniform::new(0_usize, node_count)
}

/// Selects a random edge using the given distribution for generating random NodeIDs.
///
/// Returns the selected edge in the form a of (source, target) tuple. This function assumes that
/// `distribution` always yields valid NodeIDs. The selected edge will not be a self loop, i.e., an
/// edge with equal source and target ID.
///
/// # Arguments
///
/// * `distribution` - the distribution for randomly sampling NodeIDs
/// * `rng` - the source of randomness
fn select_random_edge<R: rand::Rng>(
    distribution: dist::Uniform<usize>,
    rng: &mut R,
) -> (usize, usize) {
    let source = distribution.sample(rng);
    let mut target = distribution.sample(rng);

    while source == target {
        target = distribution.sample(rng);
    }

    (source, target)
}
