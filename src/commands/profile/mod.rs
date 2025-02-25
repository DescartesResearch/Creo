use chrono::Utc;
use creo_lib::{application::get_host, programming_language::ProgrammingLanguage};
use rand_seeder as rng;
use std::ffi::OsString;

use crate::{cli, Error, Result};

mod aggregate;
mod application;
mod benchmark;
mod deploy;
mod graph;
mod io;
mod pull;

pub use aggregate::aggregate;
pub use benchmark::benchmark;
pub use deploy::invoke;
pub use pull::pull;

pub fn generate(
    args: &cli::profile::generate::Config,
    root: impl AsRef<std::path::Path>,
) -> Result<()> {
    let root = root.as_ref();
    let root_handler_dir = root.join(creo_lib::HANDLER_FUNCTION_DIR);

    let seed = format!("profiling-{}", &args.language).to_lowercase();
    let generation_time = Utc::now();
    let gen_ts = generation_time.timestamp();
    let mut rng: rng::SipRng = rng::Seeder::from(&seed).make_rng();

    let all_defs = crate::io::glob_language_handler_definitions(&root_handler_dir, &args.language)?;
    let graph = graph::generate_graph(all_defs.len());
    let application = application::profile_application(args, graph, all_defs);

    // Create output directory (if it does not exist)
    let out_dir = root.join(creo_lib::PROFILE_DIR);
    crate::io::create_output_directory(&out_dir)?;

    // Create application directory
    let app_name = generate_profile_app_dir_name(&args.language);
    let app_dir = out_dir.join(&app_name);
    drop(out_dir);
    let service_count = application.service_count();
    let app_meta = io::create_application_meta_data(args, &app_name, service_count, &seed);
    crate::io::create_application_directory(&app_dir, app_meta)?;

    let registry = crate::io::create_handler_function_registry(&application)?;
    let template_dir = root.join(creo_lib::TEMPLATES_DIR);
    for service in application.iter_micro_services() {
        let endpoint = application
            .iter_service_endpoints(service.id)
            .next()
            .expect("should have exactly one endpoint");
        let mut dir_name = OsString::from("handler-");
        dir_name.push(
            endpoint
                .handler_dir
                .file_name()
                .expect("should be able to obtain directory name"),
        );
        let service_name = dir_name.to_string_lossy();
        crate::io::create_service_folder(
            &app_dir,
            &service_name,
            &template_dir,
            &service,
            &application,
            &registry,
            &mut rng,
        )?;
        let service_dir = app_dir.join(&dir_name);
        let mut docker_compose = creo_lib::compose::create_service_compose_with_build(
            &application,
            &service,
            &registry,
            &app_name,
            gen_ts,
        );
        let depends_on = get_host(service.id);
        crate::io::add_metrics_collection(&service_dir, vec![depends_on], &mut docker_compose)?;

        crate::io::write_docker_compose_file(service_dir.join("compose.yml"), &docker_compose)?;

        let mut has_init = false;
        for dependency in registry.get_service_dependencies(&application, service.id) {
            let mut init_names = Vec::default();
            if let Some(init_dir) = &dependency.init {
                has_init = true;
                let init_service_dir = service_dir.join("init-services").join(init_dir);
                if init_service_dir.is_dir() {
                    continue;
                }
                creo_lib::io::copy_dir_all(
                    std::path::PathBuf::from("assets/init-services/")
                        .join(service.language.as_dir_name())
                        .join(init_dir),
                    &init_service_dir,
                )
                .map_err(|err| {
                    Error::new(format!(
                        "failed to copy init dir to path `{}`!\n\tReason: {}",
                        init_service_dir.display(),
                        err
                    ))
                })?;
                init_names.push(format!(
                    "{}-{}",
                    dependency.name.as_service_name(get_host(service.id)),
                    init_dir
                ));
            }
            crate::io::create_init_service_file(
                &init_names,
                service_dir.join("init-services.conf"),
            )?;
        }
        if has_init {
            crate::io::copy_file(
                std::path::Path::new("assets/init-services/init.sh"),
                service_dir.join("init.sh"),
            )?;
        }

        let (load_generator_file, _) =
            creo_lib::io::create_load_generator_file(&application, &service, &registry);
        creo_lib::io::write_load_generator_file(
            &load_generator_file,
            service_dir.join("load_generator.lua"),
        )
        .map_err(|err| {
            Error::new(format!(
                "failed to write file for path {}!\n\tReason: {}",
                service_dir.display(),
                err
            ))
        })?;
    }

    Ok(())
}

pub fn generate_profile_app_dir_name(lang: &ProgrammingLanguage) -> String {
    format!("profile-{}", lang.as_dir_name())
}
