use crate::cli;

pub fn profile_application(
    args: &cli::profile::generate::Config,
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
