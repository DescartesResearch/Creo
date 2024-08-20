use std::collections::HashMap;

use creo_lib::{graph, handler, programming_lanuage::ProgrammingLanguage};

use crate::{cli, Result};

use rand::seq::{IteratorRandom, SliceRandom};

pub fn generate_application<R: rand::Rng>(
    root: impl AsRef<std::path::Path>,
    args: &cli::generate::Config,
    graph: graph::ColoredGraph,
    rng: &mut R,
) -> Result<creo_lib::graph::ApplicationGraph> {
    let root = root.as_ref();
    let service_types = &args.service_types;

    let handler_root_dir = root.join(creo_lib::HANDLER_FUNCTION_DIR);
    let all_defs =
        crate::io::parse_handler_definitions(handler_root_dir, args.selected_languages.iter())?;
    Ok(generate_application_graph(
        graph,
        service_types,
        all_defs,
        rng,
        args.start_port,
    ))
}

fn generate_application_graph<R: rand::Rng>(
    graph: creo_lib::graph::ColoredGraph,
    service_types: &creo_lib::ServiceTypeVec,
    mut all_defs: HashMap<ProgrammingLanguage, Vec<handler::Definition>>,
    rng: &mut R,
    start_port: u32,
) -> creo_lib::graph::ApplicationGraph {
    let mut langs: Vec<ProgrammingLanguage> = Vec::with_capacity(graph.color_count());
    let mut s_types: creo_lib::ServiceTypeVec =
        creo_lib::ServiceTypeVec(Vec::with_capacity(graph.color_count()));
    let mut defs: Vec<std::path::PathBuf> = Vec::with_capacity(graph.graph.node_count());

    for _ in graph.iter_colors() {
        let lang = all_defs.keys().choose(rng).unwrap();
        langs.push(*lang);
        let s_type = creo_lib::selection::select_service_type(&service_types.0, rng);
        s_types.0.push(s_type);
    }

    for (_, color) in graph.iter_nodes_with_colors() {
        let lang = &langs[color.0];
        let s_type = &s_types.0[color.0];
        let resource = creo_lib::selection::select_resource(s_type, rng);
        let handler_definitions = all_defs.get_mut(lang).unwrap();
        handler_definitions.sort_by(|a, b| {
            let a_utilization = a.utilization.get(&resource.resource).unwrap_or_else(|| {
                panic!(
                    "should find a utilization for resource {:?} of definition {:?}",
                    resource, a
                )
            });
            let b_utilization = b.utilization.get(&resource.resource).unwrap_or_else(|| {
                panic!(
                    "should find a utilization for resource {:?} of definition {:?}",
                    resource, b
                )
            });
            a_utilization.partial_cmp(b_utilization).unwrap()
        });
        let bucket = creo_lib::selection::select_bucket(handler_definitions, &resource);
        let definition = bucket
            .choose(rng)
            .expect("should choose a definition")
            .clone();
        defs.push(definition.directory);
    }

    creo_lib::graph::ApplicationGraph::new(graph, langs, start_port, defs)
}
