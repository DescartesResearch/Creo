pub struct MainTemplate {
    pub template_dir: &'static str,
    pub root_template_name: &'static str,
    pub auxiliry_template_names: &'static [AuxiliryTemplate],
}

pub struct AuxiliryTemplate {
    pub template_name: &'static str,
    pub file_name: &'static str,
}

pub trait MainGenerator {
    fn create_main_template(&self) -> MainTemplate;
}
