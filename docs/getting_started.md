# Getting Started

In this document, you will learn how to use Creo by generating a small sample application.
Please note the following prerequesites:

## Prerequesites

1. A working installation of the most recent version of the Rust programming language.
   You can find installation instructions [here](https://www.rust-lang.org/tools/install).
2. Ensure that `~/.cargo/bin` is added to your `$PATH` environment variable.

## Installation

1. Clone this repository to your local machine.
2. Install the project using `cargo`.

```bash
cargo install --path . --locked
```

3. Test your installation by checking the version of Creo.

```bash
creo --version
```

## Generating an Application

To generate a microservice application, we need to specify the expected topology and workload of the generated application.
For this, Creo supports three distinct modes:

- **AutoPilot**: Automatically generate a random topology and workload adhereing to the configured constraints
- **Hybrid**: Manually specify the topology and automatically generate a random application workload adhereing to
  the configured constraints.
- **Manual**: Manually specify both the topology and workload of the application

In the following, we will use the **AutoPilot** mode, since it requires the least amount of configuration.
To learn more about the **Hybrid** mode, please refer to [here](./hybrid.md).
To learn more about the **Manual** mode, please refer to [here](./manual.md).

By default, Creo uses the configuration file stored at `config/generate.yml`.
Below is an example configuration to generate a simple test application.

```yaml
# file: config/generate.yml

# `name` specifies the name of the application.
# This will be used as the name of the output directory.
name: my_first_application
# `mode` specifies the generation mode.
mode: auto_pilot
topology:
  endpoints: 3 # The total number of endpoints in the application
  inter_service_calls: 2 # The total number of inter-service calls in the application
  services: 3 # The total number of microservices in the application
workload:
  # `programming_languages` specifies the programming languages that may be used for
  # the microservices of the application.
  programming_languages: [rust, python]
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
          fraction: 100 # Probability of an endpoint to exhibit the specified
          # label-bucket combination, i.e., NETWORK_TRANSMIT-HIGH
          bucket: HIGH
```

Please note that the `topology` configuration has the following constraints:

1. The number of `inter_service_calls` must be smaller or equal to the product of the number of `endpoints` and the
   number of `services`, i.e., $\#inter\_service\_calls \le \#endpoints \cdot \#services$
2. The number of `endpoints` must be at least the number of `services`, i.e. $\#endpoints \ge \#services$

To learn more about how the generation process works in detail, please refer to [here](./architecture.md).

With the above configuration, we can generate our first test application by running the following command:

```bash
creo generate
```

If you want to use a configuration that is not stored at the default location (`config/generate.yml`), you can use:

```bash
creo generate --config <path-to-the-config>
```

With this, you should successfully generate an application and find a new directory with the application name under the
`output` directory in the project root.

## Benchmarking the Application

Now that we generated a microservice application, we can use Creo to conduct experiments with this application.
In order to conduct experiments, we require the following prerequesites:

### Prerequesites

1. Two network-accessible servers running Linux as the operating system (_Note_: The following has been only tested for
   Ubuntu server)
2. User accounts with identical usernames on both servers
3. SSH access to the servers from the machine running Creo as well as among the servers using SSH-Key authentication
   with a password-less SSH-Keypair and the user account from Prerequesite 2.
4. An installation of [docker](https://www.docker.com/) and [docker compose](https://docs.docker.com/compose/install/)
   on both servers
5. Docker CLI access without sudo for the user accounts of prerequesite 2.
6. An installation of [GNU Screen](https://www.gnu.org/software/screen/) on both servers

One of the servers acts as the master and runs the load generator to send requests to the application.
The second server is the worker host running the application using `docker compose`.

### Execution

Similar to the generation, Creo uses a configuration file as the specification for a benchmark run.
By default, Creo uses the configuration file stored at `config/benchmark.yml`.
Below is an example configuration for benchmarking our previously generated test application.

```yaml
# file: config/benchmark.yml

# `application` specifies the name of the application to benchmark.
# This is the name of the top-level application directory.
application: my_first_application
# `ssh` specifies the servers, SSH keyfile, and user name.
ssh:
  # Path to the private ssh keyfile (supports tilde/enviroment variable expansion)
  key_file: ~/.ssh/id_ed25519 # (equivalent: $HOME/.ssh/id_ed25519)
  user_name: myserveruser
  master_hosts:
    - 1.1.1.1
  worker_hosts:
    - 2.2.2.2
# `benchmark` specifies the properties of the experiment to be conducted
benchmark:
  # `warmup` specifies the warmup procedure before the experiment
  warmup:
    rate: 5 # requests per second during the warmup
    duration: 30 # duration of the warmup in seconds
    pause: 10 # duration in seconds to wait between the warmup and experiment
  # `intensity` specifies the starting and ending number of requests per second.
  # The load generator will linearily interpolate the requests per second for each time step
  # over the total experiment duration.
  intensity:
    start: 25
    end: 25
  # `duration` specifies the benchmark duration in seconds
  duration: 60
  # `timeout` specifies the number of milliseconds after which a request will timeout.
  # A timeout of `0` means no timeout.
  timeout: 8000
  # `virtual_users` specifies the number of virtual users of the load generator.
  # As a good rule of thumb, this should be set to the maximum number of requests per second
  # multiplied by the timeout (in seconds). In this case, 25 * (8000/1000) = 200
  virtual_user: 200
  # `records` specifies the number of database records to insert, if a microservice requires
  # database connection
  records: 3000000
```

Before we can start a benchmark run, we first have to deploy the application and load generator setup.
This command uses the server specifications of the above benchmark configuration.

```bash
creo deploy
```

Or, if your benchmark configuration is not stored at the default location (`config/benchmark.yml`):

```bash
creo deploy --config <path-to-the-config>
```

_Note_: This command may take a while depending on your internet connection speed.

After the deployment, we can start a benchmark run with the following command:

```bash
creo benchmark
```

Again, we can use `--config` to specify a non-default path.

```bash
creo benchmark --config <path-to-the-config>
```

### Getting the Results

After the benchmark run has finished, we can download the results using:

```bash
creo download
```

This will save the results in a directory named `benchmarks` in the output directory of the application.
