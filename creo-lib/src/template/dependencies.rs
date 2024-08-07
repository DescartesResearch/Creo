#[derive(serde::Serialize, Debug)]
pub struct DependencyData<'a> {
    pub service_name: &'a str,
    pub dependencies: Vec<String>,
}
