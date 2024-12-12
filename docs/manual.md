# Manual Generation Mode

In this document, you will learn how to use the **Manual** generation mode of Creo.
This mode allows to manually specify both the topology and workload of the application.
Below is an example configuration showing the usage of the manual generation mode.

```yaml
name: my_application
mode: manual
services:
  - name: cart
    language: python
    endpoints:
      - name: add
        inter_service_calls:
          - catalogue.product
          - catalogue.category
        function: primes
      - name: cart
        function: matrix
  - name: catalogue
    language: rust
    endpoints:
      - name: product
        # you can also specify the list inline using the flow syntax.
        # This is helpful for short lists, as it makes the definition more compact.
        inter_service_calls: [payment.checkout]
        function: rng
      - name: category
        function: invoice_create
  - name: payment
    language: python
    endpoints:
      - name: checkout
        function: user_create
```

The above configuration defines three services under the top-level `services` key.
Each service can have an arbitrary name.
In this example, the services have the names _cart_, _catalogue_, and _payment_.
The `language` key of each microservices specifies the desired programming language,
while the `endpoints` list defines the endpoints of the service.

Each endpoint definition also requires an arbitrary name that must be unique across the particular
service the endpoint belongs to. The `function` key specifies the desired _handler function_ of the
endpoint. The value should be the directory name of the handler function from the
`assets/handlers/<programming_language>` collection.
Lastly, an endpoint may specify _inter-service calls_ under the `inter_service_calls` key.
If the endpoint has no inter-service dependencies the `inter_service_calls` key may be omitted.
The items of this list can specify the call dependencies in the format `<service_name>.<endpoint_name>`.
For instance, in the above configuration, the `add` endpoint of the `cart` service, calls both endpoints
of the `catalogue` service.
