pub fn write_router_file(
    out_dir: impl AsRef<std::path::Path>,
    out_file: &str,
    template_dir: impl AsRef<std::path::Path>,
    router_file: super::RouterFileData,
    opts: super::RouterTemplate,
) -> std::io::Result<()> {
    let file = out_dir.as_ref().join(out_file);
    let template_dir = template_dir.as_ref().join(opts.template_dir);
    let hbs = register_templates(&template_dir)?;
    write_template(&hbs, opts.root_template_name, &router_file, file)?;

    Ok(())
}

pub fn write_service_call_file(
    out_dir: impl AsRef<std::path::Path>,
    out_file: &str,
    template_dir: impl AsRef<std::path::Path>,
    service_call_file: super::ServiceCallFileData,
    opts: super::ServiceCallTemplate,
) -> std::io::Result<()> {
    let file = out_dir.as_ref().join(out_file);
    let template_dir = template_dir.as_ref().join(opts.template_dir);
    let hbs = register_templates(&template_dir)?;
    write_template(&hbs, opts.root_template_name, &service_call_file, file)?;
    Ok(())
}

pub fn register_templates<'a>(
    dir: impl AsRef<std::path::Path>,
) -> std::io::Result<handlebars::Handlebars<'a>> {
    let mut hbs = handlebars::Handlebars::new();
    hbs.register_escape_fn(handlebars::no_escape);
    hbs.register_templates_directory(
        &dir,
        handlebars::DirectorySourceOptions {
            tpl_extension: ".mgt".to_owned(),
            hidden: false,
            temporary: false,
        },
    )
    .map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "could not obtain templates from path {}!\n\tReason: {}",
                dir.as_ref().display(),
                err
            ),
        )
    })?;

    Ok(hbs)
}

pub fn register_template_file(
    template_name: &str,
    file: impl AsRef<std::path::Path>,
) -> std::io::Result<handlebars::Handlebars<'_>> {
    let mut hbs = handlebars::Handlebars::new();
    hbs.register_escape_fn(handlebars::no_escape);
    hbs.register_template_file(template_name, &file)
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "could not obtain template file from path {}!\n\tReason: {}",
                    file.as_ref().display(),
                    err
                ),
            )
        })?;
    Ok(hbs)
}

fn write_template<T: serde::Serialize>(
    hbs: &handlebars::Handlebars,
    template_name: impl AsRef<str>,
    data: &T,
    file: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    if let Some(parent) = file.as_ref().parent() {
        crate::io::create_dir_all(parent)?;
    }
    hbs.render_to_write(template_name.as_ref(), data, std::fs::File::create(file)?)
        .map_err(|err| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "could not render template {}!\n\tReason: {}",
                    template_name.as_ref(),
                    err
                ),
            )
        })
}

pub fn write_main_file(
    out_dir: impl AsRef<std::path::Path>,
    out_file: &str,
    template_dir: impl AsRef<std::path::Path>,
    info: super::ServiceInfo,
    opts: super::MainTemplate,
) -> std::io::Result<()> {
    let file = out_dir.as_ref().join(out_file);
    let template_dir = template_dir.as_ref().join(opts.template_dir);
    let hbs = register_templates(&template_dir)?;
    write_template(&hbs, opts.root_template_name, &info, file)?;
    for aux in opts.auxiliry_template_names {
        write_template(
            &hbs,
            aux.template_name,
            &String::new(),
            out_dir.as_ref().join(aux.file_name),
        )?;
    }

    Ok(())
}

pub fn write_docker_file(
    out_dir: impl AsRef<std::path::Path>,
    template_path: impl AsRef<std::path::Path>,
    data: super::docker::DockerfileData,
) -> std::io::Result<()> {
    let file = out_dir.as_ref().join("Dockerfile");
    let hbs = register_template_file("Dockerfile", template_path)?;
    write_template(&hbs, "Dockerfile", &data, file)?;

    Ok(())
}

pub fn write_dependency_file(
    out_dir: impl AsRef<std::path::Path>,
    template_path: impl AsRef<std::path::Path>,
    file_name: impl AsRef<str>,
    data: super::DependencyData<'_>,
) -> std::io::Result<()> {
    let file = out_dir.as_ref().join(file_name.as_ref());
    let hbs = register_template_file("Dependency", template_path)?;
    write_template(&hbs, "Dependency", &data, file)?;

    Ok(())
}
