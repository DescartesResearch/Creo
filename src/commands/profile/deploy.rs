use std::path::PathBuf;

use futures::{future::try_join, TryFutureExt};

pub async fn invoke(config: &creo_lib::ssh::Config, app_name: String) -> crate::Result<()> {
    let src = PathBuf::from_iter::<[&std::path::Path; 2]>([
        creo_lib::PROFILE_DIR.as_ref(),
        app_name.as_ref(),
    ]);
    let prefix = src.parent().unwrap();
    let (clients, service_dirs) = try_join(
        creo_lib::ssh::establish_connections(
            config,
            config.master_hosts.iter().chain(config.worker_hosts.iter()),
        )
        .map_err(crate::Error::from),
        creo_lib::io::list_service_directories(&src).map_err(crate::Error::from),
    )
    .await?;

    let (master_clients, worker_clients) = clients.split_at(config.master_hosts.len());
    let script_templates = creo_lib::template::register_templates(creo_lib::SCRIPTS_DIR)?;
    let service_chunks = chunk_n(&service_dirs, worker_clients.len());

    for (i, chunk) in service_chunks.enumerate() {
        let worker_client = &worker_clients[i];
        let worker_ip = worker_client.get_connection_ip();
        log::info!("Creating archive for worker `{}`", &worker_ip);
        let remote_worker_path = worker_client
            .canonicalize(format!("worker-{i}-archive.tar.gz"))
            .await?;

        let local_worker_archive_path = std::env::temp_dir().join("worker-archive.tar.gz");
        let local_worker_archive = tokio::fs::File::create(&local_worker_archive_path).await?;
        let mut worker_builder =
            creo_lib::remote::archive::Builder::new_compressed(local_worker_archive);

        for dir in chunk {
            let archive_path = dir.strip_prefix(prefix).unwrap();
            worker_builder
                .append_dir_all(
                    archive_path,
                    dir,
                    &[
                        "**/benchmarks/",
                        "**/metrics/",
                        "**/user_requests.lua",
                        "**/load_generator.lua",
                    ],
                )
                .await?
        }

        worker_builder.into_inner().await?;

        log::info!("Upload archive for worker `{}`", worker_ip);
        creo_lib::remote::upload_and_extract_archive(
            worker_client,
            remote_worker_path,
            local_worker_archive_path,
        )
        .await?;
    }

    let mut service_chunks = chunk_n(&service_dirs, worker_clients.len());
    let worker_chunks = chunk_n(worker_clients, master_clients.len());

    for (master_client, worker_clients) in master_clients.iter().zip(worker_chunks) {
        let master_ip = master_client.get_connection_ip();
        log::info!("Creating archive for master `{}`", &master_ip);
        let remote_master_path = master_client.canonicalize("master-archive.tar.gz").await?;
        let local_master_archive_path = std::env::temp_dir().join("master-archive.tar.gz");
        let local_master_archive = tokio::fs::File::create(&local_master_archive_path).await?;
        let mut master_builder =
            creo_lib::remote::archive::Builder::new_compressed(local_master_archive);
        let archive_root = src.strip_prefix(prefix).unwrap();

        master_builder
            .append_dir_all("load_generator", creo_lib::LOAD_GENERATOR_DIR, &[])
            .await?;

        for worker_client in worker_clients {
            let service_dirs = service_chunks.next().unwrap();
            let worker_ip = worker_client.get_connection_ip().to_string();
            let worker_ip_dir = worker_ip.replace(".", "-");
            let archive_worker_path = archive_root.join(worker_ip_dir);
            for dir in service_dirs {
                let dir_name = dir.file_name().unwrap();
                let archive_service_dir = archive_worker_path.join(dir_name);

                // Add load generator file
                let load_file_path = dir.join("load_generator.lua");
                let load_file = tokio::fs::read_to_string(&load_file_path).await?;
                let load_file = load_file.replace("{{APPLICATION_HOST}}", &worker_ip);
                master_builder
                    .append_data_as_file(
                        archive_service_dir.join("load_generator.lua"),
                        &mut load_file.as_bytes(),
                        load_file.len() as u64,
                    )
                    .await?;
                let template_path = archive_root.join(dir_name);
                let template_path = template_path.to_string_lossy();
                let template_data = ApplicationScript {
                    user_name: &config.user_name,
                    worker_ip: &worker_ip,
                    path: &template_path,
                };

                let start_app_script =
                    script_templates.render("start_application", &template_data)?;
                master_builder
                    .append_data_as_file(
                        archive_service_dir.join("start_application.sh"),
                        start_app_script.as_bytes(),
                        start_app_script.len() as u64,
                    )
                    .await?;

                let stop_app_script =
                    script_templates.render("stop_application", &template_data)?;
                master_builder
                    .append_data_as_file(
                        archive_service_dir.join("stop_application.sh"),
                        stop_app_script.as_bytes(),
                        stop_app_script.len() as u64,
                    )
                    .await?;

                let save_metrics_script =
                    script_templates.render("save_metrics", &template_data)?;
                master_builder
                    .append_data_as_file(
                        archive_service_dir.join("save_metrics.sh"),
                        save_metrics_script.as_bytes(),
                        save_metrics_script.len() as u64,
                    )
                    .await?;

                let init_script = script_templates.render("init", &template_data)?;
                master_builder
                    .append_data_as_file(
                        archive_service_dir.join("init.sh"),
                        init_script.as_bytes(),
                        init_script.len() as u64,
                    )
                    .await?;
            }
        }

        master_builder.into_inner().await?;

        log::info!("Upload archive for master `{}`", master_ip);
        creo_lib::remote::upload_and_extract_archive(
            master_client,
            remote_master_path,
            local_master_archive_path,
        )
        .await?;
    }

    Ok(())
}

fn chunk_n<T>(slice: &[T], n: usize) -> impl Iterator<Item = &[T]> {
    let len = slice.len() / n;
    let rem = slice.len() % n;
    Split { slice, len, rem }
}

struct Split<'a, T> {
    slice: &'a [T],
    len: usize,
    rem: usize,
}

impl<'a, T> Iterator for Split<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let mut len = self.len;
        if self.rem > 0 {
            len += 1;
            self.rem -= 1;
        }
        let (chunk, rest) = self.slice.split_at(len);
        self.slice = rest;
        Some(chunk)
    }
}

#[derive(serde::Serialize)]
pub struct ApplicationScript<'a> {
    pub user_name: &'a str,
    pub worker_ip: &'a str,
    pub path: &'a str,
}
