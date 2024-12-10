use creo_lib::Port;
use lazy_errors::prelude::*;
use std::collections::HashMap;

use creo_lib::{graph, handler, programming_lanuage::ProgrammingLanguage};

use crate::{config, Result};

use rand::seq::IteratorRandom;

pub fn auto_pilot<R: rand::Rng>(
    root: impl AsRef<std::path::Path>,
    args: &config::application::AutoPilotConfig,
    graph: graph::ColoredGraph,
    start_port: Port,
    rng: &mut R,
) -> Result<creo_lib::graph::ApplicationGraph> {
    let root = root.as_ref();
    let service_types = &args.service_types;

    let handler_root_dir = root.join(creo_lib::HANDLER_FUNCTION_DIR);
    let all_defs =
        crate::io::parse_handler_definitions(handler_root_dir, args.programming_languages.iter())?;
    Ok(generate_application_graph(
        graph,
        service_types,
        all_defs,
        rng,
        start_port,
    ))
}

fn generate_application_graph<R: rand::Rng>(
    graph: creo_lib::graph::ColoredGraph,
    service_types: &creo_lib::ServiceTypeVec,
    mut all_defs: HashMap<ProgrammingLanguage, Vec<handler::Definition>>,
    rng: &mut R,
    start_port: Port,
) -> creo_lib::graph::ApplicationGraph {
    let mut langs: Vec<ProgrammingLanguage> = Vec::with_capacity(graph.color_count());
    let mut s_types = Vec::with_capacity(graph.color_count());
    let mut defs: Vec<std::path::PathBuf> = Vec::with_capacity(graph.graph.node_count());

    for _ in graph.iter_colors() {
        let lang = all_defs.keys().choose(rng).unwrap();
        langs.push(*lang);
        let s_type = creo_lib::selection::select_service_type(&service_types.0, rng);
        s_types.push(s_type);
    }

    for (_, color) in graph.iter_nodes_with_colors() {
        let lang = &langs[color.0];
        let s_type = &s_types[color.0];
        let resource = creo_lib::selection::select_resource(s_type, rng);
        let handler_definitions = all_defs.get_mut(lang).unwrap();
        let definition =
            creo_lib::selection::select_definition(handler_definitions, &resource, rng);
        defs.push(definition.directory);
    }

    creo_lib::graph::ApplicationGraph::new(graph, langs, start_port.into(), defs)
}

pub fn manual(
    graph: graph::ColoredGraph,
    languages: Vec<ProgrammingLanguage>,
    definitions: Vec<std::path::PathBuf>,
    start_port: Port,
) -> Result<creo_lib::graph::ApplicationGraph> {
    let mut errors =
        ErrorStash::new(|| "There were one or more errors during handler function assignment!");

    for def in &definitions {
        if !def.is_dir() {
            errors.push(format!(
                "invalid handler function assignment: {} is not a directory",
                def.display()
            ));
        }
    }

    errors.into_result()?;

    Ok(creo_lib::graph::ApplicationGraph::new(
        graph,
        languages,
        start_port.into(),
        definitions,
    ))
}
