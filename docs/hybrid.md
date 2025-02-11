# Hybrid Generation Mode

In this document, you will learn how to use the **Hybrid** generation mode of Creo.
This mode allows to manually specify the topology of the application, while automatically assigning the workload.
Below is an example configuration showing the usage of the hybrid generation mode.

```yaml
name: my_application
mode: hybrid
topology:
  services:
    - name: cart
      endpoints:
        - name: add
          inter_service_calls:
            - catalogue.product
            - catalogue.category
        - name: cart
    - name: catalogue
      endpoints:
        - name: product
          # you can also specify the list inline using the flow syntax.
          # This is helpful for short lists, as it makes the definition more compact.
          inter_service_calls: [payment.checkout]
        - name: category
    - name: payment
      language: python
      endpoints:
        - name: checkout
workload:
  # `programming_languages` specifies the programming languages that may be used for
  # the microservices of the application.
  programming_languages:
    [rust, python]
    # `service_types` specifies the types of microservices comprising the application.
    # In this case, we define two service types. The first type defines a microservice,
    # for which 100% of the microservice's endpoints should consume `HIGH` CPU.
    # The second type defines a microservice, for which 100% of the microservice's endpoints
    # should produce a `HIGH` outgoing network usage. Both service types have a probability of
    # 50% to occur in the application.
  service_types:
    # CPU-intensive microservice
    - fraction: 50 # Probability of this service type to occur in the application
      properties:
        - label: CPU
          fraction: 100 # Probability of an endpoint to exhibit the specified
          # label-bucket combination, i.e. CPU-HIGH
          bucket: HIGH
    # Outgoing network-intensive microservice
    - fraction: 50 # Probability of this service type to occur in the application
      properties:
        - label: NETWORK_TRANSMIT
          fraction:
            100 # Probability of an endpoint to exhibit the specified
            # label-bucket combination, i.e., NETWORK_TRANSMIT-HIGH
          bucket: HIGH
```

The above configuration defines three services under the top-level `topology` key.
Each service can have an arbitrary name.
In this example, the services have the names _cart_, _catalogue_, and _payment_.
The `endpoints` list defines the endpoints of the respective service.

Each endpoint definition also requires an arbitrary name that must be unique across the particular
service the endpoint belongs to.
Lastly, an endpoint may specify _inter-service calls_ under the `inter_service_calls` key.
If the endpoint has no inter-service dependencies the `inter_service_calls` key may be omitted.
The items of this list can specify the call dependencies in the format `<service_name>.<endpoint_name>`.
For instance, in the above configuration, the `add` endpoint of the `cart` service, calls both endpoints
of the `catalogue` service.

The `programming_languages` key under the top-level `workload` key defines the available programming languages during
the generation. In this case, all languages are equally likely, as the configuration does not specify a particular
distribution of programming languages. Alternatively, the configuration may specify a particular language distribution
in the format `language:fraction`. For instance, the following configuration would specify that 80% of the microservices
in the application are in `Python`, while only 20% are implemented in `Rust`:

```yaml
programming_languages: ["rust:20", "python:80"]
```

The `service_types` key defines the set of different service characteristics of the application. In the configuration
above, the application comprises two distinct service types. Consequently, a particular microservice in the application
is either a CPU-intensive microservice or a outgoing network-intensive microservice. Both service types are equally
likely.
