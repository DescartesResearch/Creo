# Creo

Creo is a framework for generating microservice applications for performance benchmarking.
The framework's key features are:

- fully executable microservices
- configurable application topology and resource usage profiles
- built-in support for standardized monitoring, load generation, and deployment

## Getting started

1. [Install](https://www.rust-lang.org/tools/install) the Rust programming language
2. Build the project with `cargo` and add the binary to your `PATH`

```shell
$ cargo build --release
$ export PATH="$PWD/target/release:$PATH"
```

3. Use `config/generate.yml` to specify the desired application.

```yaml
programming_languages:
  - rust
  - python
application_name: my_microservice_application
services: 2
endpoints: 10
service_calls: 4
service_types:
  - fraction: 50
    resources:
      - resource: CPU
        intensity: HIGH
        fraction: 100
  - fraction: 50
    resources:
      - resource: NETWORK_TRANSMIT
        intensity: LOW
        fraction: 100
```

4. Generate the microservice application

```shell
$ creo generate
```

5. Setup two Linux Servers (only tested on Ubuntu Server) with the following requirements:
   - Setup users with same usernames
   - Setup ssh key authentication with the same, password-less ssh key for both users
   - Install [`docker`](https://www.docker.com/) and [`docker compose`](https://docs.docker.com/compose/install/)
     on both servers

One server acts as the master running the load generator sending requests to the application.
The second servers is the worker running the application using `docker compose`.

6. Use `config/benchmark.yml` to specify the deployment and experiment configuration.

```yaml
ssh:
  # Location to your keyfile (supports environment variable/tilde expansion)
  key_file: ~/.ssh/id_ed25519
  master_hosts:
    - 1.1.1.1
  worker_hosts:
    - 2.2.2.2
  user_name: myuser
application: my_microservice_application
benchmark:
  records: 3000000
  virtual_user: 450
  timeout: 3000
  duration: 600
  warmup:
    rate: 10
    pause: 10
    duration: 120
  intensity:
    start: 150
    end: 150
```

7. Deploy the application and load generation setup.

```shell
$ creo deploy
```

8. Start the experiment.

```shell
$ creo benchmark
```
