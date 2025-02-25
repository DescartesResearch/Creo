# Handler Function Dependencies

Handler functions may depend on one or more handler function dependencies. The dependencies are independent containers
that run adjacent to the handler function's microservice container. For instance, a handler function requiring a
database connection, may specify a database dependency in its `definition.yml` file.

## Adding new dependencies

In `creo-lib/src/dependencies/mod.rs` check if the dependency you want to add fits into an existing `DependencyType`.
If yes, navigate to the corresponding sub-module and complete the steps below. If no existing `DependencyType` fits,
add a new variant to the `DependencyType` enum and create the corresponding sub-module. You can use existing sub-module
as a template. Adjust the corresponding `DependencyType` methods in `creo-lib/src/dependencies/mod.rs` to include the
new variant, after you have added the respective implementations to the new sub-module.

### Adding the dependency variant

In the sub-module of the corresponding `DependencyType`, add the variant for the new variant to the specific dependency
enum. You may create additional sub-sub-modules for the implementations for this new variant. The following
implementations are required:

- `as_service_name()`: This function should return a valid `docker compose` service name based on the given unique
  `service_name` argument
- `as_docker_compose_service()`: This function should return a tuple of the service name (as per `as_service_name()`) and
  the respective `docker compose` service. The `docker compose` service should specify a publicly available image and must
  not specify a `build` property.
- `as_docker_compose_environment()`: This function should return a vector of strings defining the key-value pairs of
  environment variables passed to the microservice container. For more details, please refer to the next section.
- `as_volume_name()`: This function is only needed if the dependency uses `volumes` and should return a unique volume
  name based on the given `service_name`.

### Passing information through environment variables

The implementation of the handler function may require certain information from the dependency it depends on. For
instance, the handler function may need the access credentials for its database client. Thus, the handler function
implementation may expect certain environment variables. Please refer to the list below for a complete list of
environment variables passed by each dependency.

#### Database Dependencies

##### Mongo

- `DB_MONGO_HOST`: contains the host name of the MongoDB database
- `DB_MONGO_PORT`: contains the port of the MongoDB database
- `DB_MONGO_USER`: contains the username of the MongoDB user
- `DB_MONGO_PASSWORD`: contains the password of the MongoDB user

### Init services

For some dependencies (such as databases) it may be useful to specify a `init-service` that runs once before the
application startup (e.g., seeding the database with records). Depending on the type of dependency the `init-service`
may expect the following environment variables. In addition, the environment variables listed for the specific
dependency variant are also accessible.

#### Database Dependencies

- `MG_SEED_COUNT`: contains the number of records to insert into the database
