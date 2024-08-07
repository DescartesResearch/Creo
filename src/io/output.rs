use std::io::Write;

use creo_lib::handler;

use crate::{Error, Result};

pub fn copy_file(
    source: impl AsRef<std::path::Path>,
    destination: impl AsRef<std::path::Path>,
) -> Result<()> {
    std::fs::copy(source.as_ref(), destination.as_ref()).map_err(|err| {
        Error::new(format!(
            "failed to copy file `{}` to path `{}`!\n\tReason: {}",
            source.as_ref().display(),
            destination.as_ref().display(),
            err
        ))
    })?;
    Ok(())
}

pub fn create_init_service_file(
    init_service_names: &[String],
    path: impl AsRef<std::path::Path>,
) -> Result<()> {
    let mut content = init_service_names.join("\n");
    content.push('\n');
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|err| {
            Error::new(format!(
                "failed to create/open init service file for path `{}`!\n\tReason: {}",
                path.as_ref().display(),
                err
            ))
        })?;
    file.write_all(content.as_bytes()).map_err(|err| {
        Error::new(format!(
            "failed to write contents for path `{}`!\n\tReason: {}",
            path.as_ref().display(),
            err
        ))
    })?;
    Ok(())
}

pub fn create_output_directory(out_dir: impl AsRef<std::path::Path>) -> Result<()> {
    if !out_dir.as_ref().exists() {
        creo_lib::io::create_dir_all(&out_dir).map_err(|err| {
            Error::new(format!(
                "failed to create output directory at path {}!\n\tReason: {}",
                out_dir.as_ref().display(),
                err
            ))
        })?
    }

    Ok(())
}

pub fn create_application_directory(
    app_dir: impl AsRef<std::path::Path>,
    app_meta: creo_lib::io::ApplicationMetaData,
) -> Result<()> {
    creo_lib::io::create_application_directory(&app_dir, app_meta).map_err(|err| {
        Error::new(format!(
            "failed to create application directory at path {}!\n\tReason: {}",
            app_dir.as_ref().display(),
            err
        ))
    })?;
    Ok(())
}

pub fn create_service_folder<R: rand::Rng>(
    application_dir: impl AsRef<std::path::Path>,
    service_name: impl AsRef<str>,
    template_dir: impl AsRef<std::path::Path>,
    service: &creo_lib::application::MicroService,
    application: &creo_lib::graph::ApplicationGraph,
    registry: &handler::FunctionRegistry,
    rng: &mut R,
) -> Result<()> {
    creo_lib::io::create_service_folder(
        application_dir.as_ref(),
        service_name.as_ref(),
        template_dir.as_ref(),
        service,
        application,
        registry,
        rng,
    )
    .map_err(|err| {
        Error::new(format!(
            "failed to create service directory for path {}!\n\tReason: {}",
            application_dir
                .as_ref()
                .join(service_name.as_ref())
                .display(),
            err
        ))
    })?;

    Ok(())
}

pub fn write_docker_compose_file(
    file: impl AsRef<std::path::Path>,
    compose: &creo_lib::compose::Compose,
) -> Result<()> {
    creo_lib::io::write_docker_compose_file(file.as_ref(), compose).map_err(|err| {
        Error::new(format!(
            "failed to write docker-compose.yml for path {}!\n\tReason: {}",
            file.as_ref().display(),
            err
        ))
    })
}

pub fn add_metrics_collection(
    dir: impl AsRef<std::path::Path>,
    depends_on: Vec<String>,
    compose: &mut creo_lib::compose::Compose,
) -> Result<()> {
    creo_lib::metrics::add_metrics_collection(&dir, depends_on, compose).map_err(|err| {
        Error::new(format!(
            "failed to add metrics collection for path {}!\n\tReason: {}",
            dir.as_ref().display(),
            err
        ))
    })
}
