use chrono::Utc;
use creo_lib::application::get_host;
use rand_seeder as rng;

use crate::{cli, util::DigitExt, Error, Result};

mod application;
mod graph;
mod io;
mod validate;

pub fn generate<P: AsRef<std::path::Path>>(args: &cli::generate::Config, root: P) -> Result<()> {
    let root = root.as_ref();
    validate::validate_arguments(args)?;

    let mut rng: rng::SipRng = rng::Seeder::from(&args.seed).make_rng();
    let generation_ts = Utc::now().timestamp();

    let graph = if args.service_call_list.is_empty() {
        graph::generate_graph(args, &mut rng)?
    } else {
        graph::generate_graph_with_edges(args)?
    };

    let application = application::generate_application(root, args, graph, &mut rng)?;

    // Create output directory (if it does not exist)
    let out_dir = root.join(creo_lib::OUTPUT_DIR);
    crate::io::create_output_directory(&out_dir)?;

    // Create application directory
    let app_dir = out_dir.join(&args.app_name);
    drop(out_dir);
    crate::io::create_application_directory(&app_dir, args.into())?;

    let digits = application.service_count().digits();
    let registry = crate::io::create_handler_function_registry(&application)?;
    let template_dir = root.join(creo_lib::TEMPLATES_DIR);
    let mut service_compose = Vec::with_capacity(application.service_count());
    let mut depends_on = Vec::with_capacity(application.service_count());
    let mut load_files = Vec::with_capacity(application.service_count());
    let mut user_files = Vec::with_capacity(application.service_count());
    let mut application_init = Vec::default();
    for service in application.iter_micro_services() {
        let dir_name = service.as_dir_name(digits);
        crate::io::create_service_folder(
            &app_dir,
            &dir_name,
            &template_dir,
            &service,
            &application,
            &registry,
            &mut rng,
        )?;
        let service_dir = app_dir.join(&dir_name);
        let docker_compose = creo_lib::compose::create_service_compose_with_build(
            &application,
            &service,
            &registry,
            &args.app_name,
            generation_ts,
        );
        crate::io::write_docker_compose_file(
            service_dir.join("docker-compose.yml"),
            &docker_compose,
        )?;
        for dependency in registry.get_service_dependencies(&application, service.id) {
            let mut init_names = Vec::default();
            if let Some(init_dir) = &dependency.init {
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
                    dependency.name.as_service_name(&dir_name),
                    init_dir
                ));
            }
            crate::io::create_init_service_file(
                &init_names,
                service_dir.join("init-services.conf"),
            )?;
            application_init.extend(init_names)
        }
        service_compose.push((dir_name, docker_compose));
        depends_on.push(get_host(service.id));
        let (load_generator_file, user_file) = creo_lib::io::create_load_generator_file(
            &application,
            &service,
            &registry,
            creo_lib::application::get_host(service.id),
        );
        creo_lib::io::write_load_generator_file(
            &load_generator_file,
            service_dir.join("load_generator.yml"),
        )
        .map_err(|err| {
            Error::new(format!(
                "failed to write file for path {}!\n\tReason: {}",
                service_dir.display(),
                err
            ))
        })?;
        creo_lib::io::write_load_generator_file(&user_file, service_dir.join("user_requests.yml"))
            .map_err(|err| {
                Error::new(format!(
                    "failed to write file for path {}!\n\tReason: {}",
                    service_dir.display(),
                    err
                ))
            })?;
        load_files.push(load_generator_file);
        user_files.push(user_file);

        crate::io::copy_file(
            std::path::Path::new("assets/init-services/init.sh"),
            service_dir.join("init.sh"),
        )?;
    }
    let mut application_compose = creo_lib::compose::create_application_compose(service_compose);
    crate::io::add_metrics_collection(&app_dir, depends_on, &mut application_compose)?;
    crate::io::create_init_service_file(&application_init, app_dir.join("init-services.conf"))?;
    crate::io::copy_file(
        std::path::Path::new("assets/init-services/init.sh"),
        app_dir.join("init.sh"),
    )?;

    crate::io::write_docker_compose_file(app_dir.join("docker-compose.yml"), &application_compose)?;
    let app_load_file = creo_lib::io::create_application_load_file(load_files);
    creo_lib::io::write_load_generator_file(&app_load_file, app_dir.join("load_generator.yml"))
        .map_err(|err| {
            Error::new(format!(
                "failed to write file for path {}!\n\tReason: {}",
                app_dir.display(),
                err
            ))
        })?;
    let app_user_file = creo_lib::io::create_application_load_file(user_files);
    creo_lib::io::write_load_generator_file(&app_user_file, app_dir.join("user_requests.yml"))
        .map_err(|err| {
            Error::new(format!(
                "failed to write file for path {}!\n\tReason: {}",
                app_dir.display(),
                err
            ))
        })?;

    Ok(())
}
