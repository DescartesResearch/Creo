use crate::cli;

impl<'a> From<&'a cli::generate::Config> for creo_lib::io::ApplicationMetaData<'a> {
    fn from(val: &'a cli::generate::Config) -> Self {
        creo_lib::io::ApplicationMetaData {
            application_name: &val.app_name,
            seed: &val.seed,
            ports: creo_lib::io::Ports {
                start: val.start_port,
                end: val.start_port + (val.number_of_services as u32),
            },
        }
    }
}
