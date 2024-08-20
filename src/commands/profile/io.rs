use crate::cli;

pub fn create_application_meta_data<'a>(
    args: &cli::profile::generate::Config,
    app_name: &'a str,
    service_count: usize,
    seed: &'a str,
) -> creo_lib::io::ApplicationMetaData<'a> {
    creo_lib::io::ApplicationMetaData {
        application_name: app_name,
        seed,
        ports: creo_lib::io::Ports {
            start: args.start_port,
            end: args.start_port + service_count as u32,
        },
    }
}
