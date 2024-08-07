use crate::cli;

pub fn generate_application(
    args: &cli::profile::GenerateConfig,
    graph: creo_lib::graph::ColoredGraph,
    defs: Vec<std::path::PathBuf>,
) -> creo_lib::graph::ApplicationGraph {
    creo_lib::graph::ApplicationGraph::new(
        graph,
        vec![args.language; defs.len()],
        args.start_port,
        defs,
    )
}
