use lazy_errors::prelude::ErrorStash;

#[derive(Debug, serde::Deserialize)]
pub struct AutoPilotConfig {
    /// The number of endpoints (vertices) to generate.
    #[serde(alias = "endpoints")]
    pub number_of_endpoints: usize,

    /// The number of inter-service calls (edges) to generate.
    #[serde(alias = "inter_service_calls")]
    pub number_of_inter_service_calls: usize,

    /// The number of services (colors) to generate.
    #[serde(alias = "services")]
    pub number_of_services: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct HypridConfig {
    /// The list of microservice definitions
    pub services: creo_lib::de::UniqueVec<MicroserviceTopologyDefinition>,
}

#[derive(Debug, serde::Deserialize, Eq)]
/// A manual definition of the topology of a microservice.
pub struct MicroserviceTopologyDefinition {
    /// The name of the microservice.
    pub name: creo_lib::de::NonEmptyString,
    /// The endpoints of the microservice.
    pub endpoints: creo_lib::de::NonEmptyVec<VertexDefinition>,
}

impl PartialEq for MicroserviceTopologyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for MicroserviceTopologyDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, serde::Deserialize, Eq)]
#[serde(remote = "Self")]
/// A manual definition of a graph vertex.
pub struct VertexDefinition {
    /// The ID of the vertex
    pub name: creo_lib::de::NonEmptyString,
    /// The IDs of the vertices this vertex is connected to
    #[serde(default)]
    pub inter_service_calls: creo_lib::de::UniqueVec<String>,
}

impl PartialEq for VertexDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for VertexDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'de> serde::Deserialize<'de> for VertexDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = VertexDefinition::deserialize(deserializer)?;

        let mut errors = ErrorStash::new(|| {
            format!(
                "There were one or more errors in the definition of endpoint {}",
                this.name.as_ref(),
            )
        });

        for call in this.inter_service_calls.iter() {
            if !call.contains(".") {
                let msg = format!("invalid inter-service call definition: {}, (expected format: <service_name>.<endpoint_name>)", call);
                errors.push(msg);
            }
        }

        errors.into_result().map_err(serde::de::Error::custom)?;
        Ok(this)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ManualConfig {
    /// The list of microservice definitions
    pub services: creo_lib::de::NonEmptyVec<MicroserviceDefinition>,
}

#[derive(Debug, serde::Deserialize, Eq)]
/// A manual definition of a microservice.
pub struct MicroserviceDefinition {
    /// The name of the microservice.
    pub name: creo_lib::de::NonEmptyString,
    /// The programming language of the microservice
    pub language: creo_lib::programming_lanuage::ProgrammingLanguage,
    /// The endpoints of the microservice.
    pub endpoints: creo_lib::de::NonEmptyVec<EndpointDefinition>,
}

impl PartialEq for MicroserviceDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for MicroserviceDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash)]
/// A manual endpoint definition
pub struct EndpointDefinition {
    #[serde(flatten)]
    /// The vertex definition of the endpoint
    pub vertex: VertexDefinition,
    /// The function directory name of the endpoint
    pub function: String,
}
