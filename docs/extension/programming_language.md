# Adding a Programming Language

Creo allows generating microservice applications in multiple programming languages. We envision an open-source
community that adds support to new programming languages to enrich the capabilities of the generator. This document
describes the steps for adding support for a new programming language. Creo requires the following language-specific
requirements:

- A `symbol generator` for generating valid symbol names in the target language
- A `data type mapper` that maps the language agnostic JSON schema data types to language-specific data types
- A `file name generator` for generating filenames
- A `Dockerfile` template for creating a microservice's Docker image

## Microservice Directory Structure

This section briefly describes the directory structure, Creo generates for each microservice. This information will be
important at several points, such as the `Dockerfile` template or the `SymbolGenerator` implementation. The directory
structure looks as follows:

```
.
├── [service-name]/
│   ├── lib/
│   │   ├── [handler-function-1]/
│   │   │   └── ...
│   │   ├── [handler-function-2]/
│   │   │   └── ...
│   │   └── ...
│   ├── src/
│   │   ├── main.[ext]
│   │   ├── router.[ext]
│   │   ├── service_calls.[ext]
│   │   └── ...
│   ├── Dockerfile
│   ├── [requirements-file]
│   └── ...
└── ...
```

The directory of a microservice contains a `Dockerfile` that is generated based on the `Dockerfile` template of the
programming language. The `lib` directory of the microservice root directory contains the directories of the handler
functions that are required by the microservice implementation. The handler function directories are copied as is
from the corresponding language directory of the `assets/handlers` path. For instance, if a `Python` microservice
requires the `hash` handler function, the directory `assets/handlers/hash` would be directly copied into the
`[service-name]/lib` path, i.e., `[service-name]/lib/hash`. The `src` directory contains the generated source code
files. All files in this directory have the language-specific extension `[ext]` (i.e., `.py` for `Python` files).

## Dockerfile Template

Every language must provide a language-specific `Dockerfile` template to build a Docker image for a microservice.
The previous section shows the location of `Dockerfile` in the generated output directory. The build context will be
will be the parent directory of the `Dockerfile`, i.e., the microservice root directory.

The Dockerfile should copy the `src` and `lib` directories as well as the language-specific `[requirements-file]` into
the image. Further, all requirements of the `[requirements-file]` should be installed. Lastly, the working directory
should be set to the `src` directory. The following two instructions should be the last instructions of the `Dockerfile`
template:

```Dockerfile
EXPOSE 80

ENTRYPOINT {{entrypoint}}
```

The `Dockerfile` template should be added to the corresponding programming language directory in the `assets/templates`
directory. For instance, the path of the `Dockerfile` template for `Python` is `assets/templates/python/Dockerfile.mgt`.

## Setup

Locate the `generator` module of the `creo-lib` library. Then add a directory with the name of the programming language
containing an empty `mod.rs` file to that module. For instance, suppose you added support for `Python`. In this case,
the `mod.rs` file should be located at `creo-lib/src/generator/python/mod.rs`.

Then add the newly created module to `creo-lib/src/generator/mod.rs`. Continuing the example from above, we would the
following line to `creo-lib/src/generator/mod.rs`:

```rust
pub mod python;
```

## Trait Implementations

Next, create a `data_type.rs`, `file_name.rs`, `local_deps.rs`, and `symbol.rs` file in the newly added programming
language module and add them to the corresponding `mod.rs` file. These files should contain the following trait
implementations (_Note_: You may use the existing implementations from already supported languages as a reference):

- `symbol.rs`: This file should contain the implementation of the `SymbolGenerator` trait. This trait allows generating
  dynamic names that follow the programming language specific conventions and rules. For example, this trait generates
  unique function names for all inter-service call functions and the correct import statements based on the
  `service_calls` file location. You may start to implement the trait like this:

  ```rust
  use crate::generator::core;

  pub struct SymbolGenerator;

  impl core::SymbolGenerator for SymbolGenerator {
      // implementation here
  }
  ```

- `data_type.rs`: This file should contain the implementation of the `DataTypeMapper` trait. This trait maps the
  language-agnostic JSON schema data types to the language-specific data types. The required methods should return the
  data type name of the corresponding type. The values produced by this trait are later used as the data types passed
  to the source code templates. You may start to implement the trait like this:

  ```rust
  use crate:{
      generator::core::{self, LanguageDataType},
      template::Import,
  };

  pub struct DataTypeMapper;

  impl core::DataTypeMapper for DataTypeMapper {
      // implementation here
  }
  ```

- `file_name.rs`: This file should contain the implementation of the `FileNameGenerator` trait. The returned file name
  path should be relative to the microservice root directory and include the `src` directory. Refer to existing
  implementations for an example. You may start to implement the trait like this:

  ```rust
  use crate::generator::core::{self, FileName};

  pub struct FileNameGenerator;

  impl core::FileNameGenerator for FileNameGenerator {
      // implementation here
  }
  ```

- `local_deps.rs`: This file should implement a function `get_local_handler_dependencies`. This function should create
  vector of strings containing the local dependency specifications. The strings of this vector should be valid local
  path dependencies in the correct language-specific format. The function receives the path to `lib` directory
  containing the folders of the chosen handler functions. This function may assume that the directory names in the `lib`
  directory are valid `UTF-8` and thus may be converted with `to_str()`. Refer to existing implementations for examples.
  You may start to implement the trait like this:

  ```rust
  pub fn get_local_handler_dependencies(
      lib_dir: impl AsRef<std::path::Path>,
  ) -> std::io::Result<Vec<String>> {
      // implementation here
  }
  ```

## Dependency File Template

Next add a dependency file template to the `templates/[programming_language]` directory. This file
should specify the dependencies that the generated service has. The following data will be provided to the dependency
file template:

```rust
struct DependencyData<'a> {
  /// the name of the generated service (can be used as the project name)
  service_name: &'a str,
  /// the vector of dependency statements in the language-specific dependency format
  dependencies: Vec<String>
}
```

For instance, this template should produce a valid `requirements.txt` file for `Python`, a valid `Cargo.toml` file for
`Rust`, or a valid `go.mod` file for `Go`.

## Constants

The following constants must be specified in the `mod.rs` file of the programming language:

- `DOCKERFILE_TEMPLATE_PATH`: the path to the Dockerfile template relative to the `assets/templates` directory.
- `DEPENDENCY_FILE_TEMPLATE_PATH`: the path to the dependency file template relative to the `assets/templates` directory.
- `DEPENDENCY_FILE_NAME`: the name of the outputted dependency file. For instance, this is `requirements.txt` for `Python`,
  `Cargo.toml` for `Rust`, or `go.mod` for `Go`.

## Programming Language Module

Finally, the newly added implementations of the `generator` module need to be added to the `programming_language` module
of the `creo-lib` library. For this add the respective language to the `ProgrammingLanguage` enum in the `mod.rs` of the
`programming_language` module (path: `creo-lib/src/programming_language/mod.rs`).
You can simply copy an existing variant and rename it.
Next adjust the following function implementations:

- `fn as_dir_name()` in `programming_language/mod.rs`
- `fn as_fraction()` in `programming_language/mod.rs`
- `fn fmt()` of the `Display` trait in `programming_language/mod.rs`
- `fn from_str()` of the `FromStr` trait in `programming_language/mod.rs`: you only need to add the new programming
  language to the match statement
- `fn to_data_type_mapper()` in `programming_language/data_type.rs`
- `fn get_local_handler_dependencies()` in `programming_language/dependency.rs`
- `fn dependency_file_name()` in `programming_language/dependency.rs`
- `fn as_dependency_file_template_path()` in `programming_language/dependency.rs`
- `fn as_docker_template_path()` in `programming_language/docker.rs`
- `fn to_file_name_generator()` in `programming_language/file.rs`
- `fn to_symbol_generator()` in `programming_language/symbol.rs`
- `fn choose_random_framework()` in `programming_language/random.rs`: This function requires the implementation of at
  least one HTTP server framework for the respective programming language. Please refer to our
  [framework extension guide](./frameworks.md) for instructions on how to add a framework.
