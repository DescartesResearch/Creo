# Adding a Web Framework

Creo uses a set of web frameworks per language to generate the necessary source code for a particular microservice.
A framework must provide the following requirements:

- Source code templates to dynamically generate valid code
- Code for generating fake data based on the signature of called handler functions
- Docker entrypoint for the Dockerfile

## New Language Setup

The steps in this section only apply if you add the first framework for a particular programming language.
If the programming language of your framework already includes other frameworks, this section may be skipped.

Locate the respective programming language module inside the `generator` module of the `creo-lib` library. Then add
a `frameworks` directory containing a empty `mod.rs` file to that module. For instance, suppose you added support for
`Python` and want to add the first framework. In this case, the `mod.rs` file should be located at
`creo-lib/src/generator/python/frameworks/mod.rs`.

In the newly created `mod.rs` file, add a `Frameworks` enum. The file may look like this:

```rust
use rand_derive::Rand;

#[derive(Rand)]
pub enum Frameworks {
    // your framework variant here, e.g.,
    // FastAPI
}
```

## Source Code Templates

Each framework must provide its own source code templates for generating the necessary microservice code based on the
generated application graph. Details about how the application is generated can be found in the
[Architecture and Design guide](../architecture.md). In general, the templates should include at least three files:

- The `main.mgt` template should generate the main file of the microservice that provides the entrypoint of the
  application
- The `router.mgt` template generates the framework-specific source code for including all endpoints of the
  microservice.
- The `service_calls.mgt` template generates the source code for performing the inter-service calls of the endpoints.

We use the [Handlebars Template Engine](https://handlebarsjs.com/) for generating the source code. If you are not
already familiar with Handlebars templates, we recommend to work through the Handlebars
[language guide](https://handlebarsjs.com/guide/#what-is-handlebars) before continuing with this guide.
In general, working with source code templates can be quite complex, so we recommend using the existing templates as
guidance.

Create the following directory structure:

```
.
├── assets/
│   ├── templates/
│   │    ├── [framework_name]/
│   │    │   ├── main.mgt
│   │    │   ├── router/
│   │    │   │   └── router.mgt
│   │    │   └── service_calls/
│   │    │       └── service_calls.mgt
│   │    └── ...
│   └── ...
└── ...
```

We separate the `router.mgt` and `service_calls.mgt` templates into distinct sub-directories to facilitate the usage of
partial templates. All additional `.mgt` files in these directories are automatically available as partial templates.

### Main Template

The `main.mgt` template is responsible for generating the application entrypoint. This template receives a
`ServiceInfo` object as input. The exact structure of the input object can be found in `creo-lib/src/template/info.rs`.
The information included in this object only relates to optional information and may be omitted in the template.
In addition, any number of auxiliary template files may be added to the framework directory. These templates do not
receive any input. However, this allows including static source code files, for better project organization.

### Router Template

The `router.mgt` template is responsible for including all endpoints of the microservice into the generated source code.
This template receives a `RouterFileData` object as input. The exact structure of the input object can be found in
`creo-lib/src/template/router/models.rs`. In short, the input comprises the necessary input statements and dynamic
information for the endpoint operations of the microservice.

An endpoint operation should implement the following logic:

1. Parse the respective handler function input from the HTTP request. In the case of a HTTP `GET` request, the function
   input only includes primitive data types. Thus, the request body should be empty and all input arguments are included
   in the URL query parameters of the request. HTTP `POST` requests include a complex data type (i.e., a object or array)
   in the request body. The request body should be passed as the raw byte string as the last argument of the handler
   function.
2. Call the respective handler function with the parsed request input. If the handler function has a return type, it
   should be included in the JSON response of the endpoint operation. Otherwise, the endpoint should return an empty
   response.
3. Possibly perform the inter service calls of the endpoint. The implementation is free to choose at what point in the
   code it calls the inter service call function. However, it must wait for the result of the inter service calls,
   before returning a response.

### Service Calls Template

The `service_calls.mgt` template is responsible for generating the fake data for the inter service calls and for
exposing a function for each endpoint with at least one inter service call to send the respective request(s).
This template receives a `ServiceCallFileData` object as input. The exact structure of the input object can be found in
`creo-lib/src/template/service_calls/models.rs`.

The service call template should implement the following logic:

1. Provide a function to generate fake data for each of the primitive and complex data types. A popular library for
   generating fake data is _Faker_. It is available for most programming languages and facilitates the generation of
   primitive data types. For objects, we can generate a fake data for each object property for the corresponding data
   type of the property. For arrays, we can generate a random number of elements with the element's data type fake
   function.
2. Provide a function that generates the necessary query parameters for each inter service call.
3. Provide a function that performs a single inter service call from endpoint A to endpoint B. The request must include
   random fake data that matches the expected input of endpoint B's handler function.
4. Provide a function that performs all inter service calls for a given endpoint A. This function must be accessible in
   the `router` source code file.

## Trait Implementations

The integration of a framework requires the implementation of several traits to provide meta-information and specific
template logic. First create a `[framework_name].rs` file in the `frameworks` module of the respective programming
language inside the `generator` module. Add this file, as a module in the `mod.rs` file of the `frameworks` directory.
For instance, to add the `Python` framework Flask, we add a `flask.rs` file at the path
`creo-lib/src/generator/python/frameworks/flask.rs` and add the following line to
`creo-lib/src/generator/python/frameworks/mod.rs`:

```rust
mod flask;
```

The following traits should be implemented in the newly created file:

- `Fakeable`: The `Fakeable` trait provides the function names and arguments for generating fake data. As mentioned
  above, we recommend to use the implementation of the popular `Faker` library for the respective programming language
  in order to generate fake primitive data types. You may start to implement the trait like this:

  ```rust
  use crate::template;

  pub struct Faker;

  impl template::Fakeable for Faker {
      // implementation here
  }
  ```

  The required trait methods, e.g., `get_string_fake` or `get_integer_fake`, should return the name and arguments of
  the function that generates the respective data type based on the given OpenAPI schema type definition. The
  `get_object_fake` and `get_array_fake` functions usually return the given unique function name as is.

- `RouterGenerator`: The `RouterGenerator` trait specifies the location of the router template files.
  Please note that the `root_template_name` does not include the `.mgt` extension.
  You may start to implement the trait like this:

  ```rust
  pub struct RouterGenerator;

  impl template::RouterGenerator for RouterGenerator {
      // implementation here
  }
  ```

- `ServiceCallGenerator`: The `ServiceCallGenerator` trait specifies the location of the service call template
  files. Please note that the `root_template_name` does not include the `.mgt` extension.
  You may start to implement the trait like this:

  ```rust
  pub struct ServiceCallGenerator;

  impl template::ServiceCallGenerator for ServiceCallGenerator {
      // implementation here
  }
  ```

- `MainGenerator`: The `MainGenerator` trait allows specifying the location of the main template file. In addition, this
  trait may specify any auxiliry files that should be generated by specifying their template name and output file name.
  Please note that the `root_template_name` does not include the `.mgt` extension.
  You may start to implement the trait like this:

  ```rust
  pub struct MainGenerator;

  impl template::MainGenerator for MainGenerator {
      // implementation here
  }
  ```

## Functions

The following functions should be implemented:

- `fn get_framework_dependencies()`: This function should return a `Vec<&'static str>` specifying the required
  dependencies for the framework in the language specific dependency format.

## Constants

The following constants should be specified:

- `DOCKER_ENTRYPOINT`: A string specfiying the framework specific docker entrypoint command in order to start the
  framework server on port 80

## Framework Generator

After adding the framework specific implementations, the last steps is to implement the `FrameworkGenerator` trait for
the `Frameworks` enum in the `frameworks` module. The function should return the correct trait implementation based on
the enum variant. The starting implementation may look like this:

```rust
use rand_derive::Rand;

use crate::{generator::core::FrameworkGenerator, template};

#[derive(Rand)]
pub enum Frameworks {
    // Variants
}

impl FrameworkGenerator for Frameworks {
    fn to_faker(&self) -> Box<dyn template::Fakeable> {
        match self {
            // ...
        }
    }

    fn to_router_generator(&self) -> Box<dyn template::RouterGenerator> {
        match self {
            // ...
        }
    }

    fn to_service_calls_generator(&self) -> Box<dyn template::ServiceCallGenerator> {
        match self {
            // ...
        }
    }

    fn to_main_generator(&self) -> Box<dyn template::MainGenerator> {
        match self {
            // ...
        }
    }

    fn get_framework_requirements(&self) -> Vec<&'static str> {
        match self {
            // ...
        }
    }

    fn get_docker_entrypoint(&self) -> &'static str {
        match self {
            // ...
        }
    }
}
```

You can refer to existing implementations (e.g. for `Python`) in the respective `frameworks` module.
