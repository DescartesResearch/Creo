# Profiling Handler Functions

This document describes the required steps for profiling handler functions of a given programming language.
We use profiling to determine the resource usage labels of all handler functions.
To ensure comparability, it is important to measure all profiling benchmark in the same environment.

## Requirements

Please ensure you fulfill the following requirements before proceeding:

- At least two hosts running Linux as their operating system (currently Ubuntu and Debian are tested)
- SSH access to all remote host with identical usernames from your local machine
- SSH access from all master hosts to all hosts servers with the identical username using passwordless
  SSH key authentication (we will cover the difference between master and worker hosts later)
- Remote hosts must all use UTC as local timezone
- Remote hosts must support SFTP
- Useful links:
  - <a href="https://www.digitalocean.com/community/tutorials/how-to-set-up-ssh-keys-on-ubuntu-20-04" target="_blank">How to Set Up SSH Keys on Ubuntu 20.04</a>

## Generate Profiling Applications

To generate all profiling applications for a specific programming language, we need to specify a profiling configuration
under `config/profile.yml`. The following shows an example configuration. For now, we are only concerned with the
`programming_language` key:

```yaml
programming_language: python
application: profile-python
ssh_config:
  key_file: ~/.ssh/id_ed25519
  master_hosts:
    - 1.1.1.1
  worker_hosts:
    - 2.2.2.2
  user_name: myremoteuser
benchmark:
  rps:
    - 25
    - 75
    - 150
  iterations: 5
  benchmark_duration: 900 # in s
  virtual_users: 450
  warmup_pause: 10 # in s
  warmup_duration: 120 # in s
  warmup_rps: 10
  timeout: 8000 # in ms
  records: 3000000
```

Generate the profiling applications with the following command:

```sh
creo profile generate
```

## Deployment

To deploy the profiling applications, we need to provide the `application` and `ssh_config` configuration parameters.
The `application` key should specify, the directory name of the generated application in `profiling` (by default this
is `profile-<language>` where `<language>` is the language name). The `master_hosts` and `worker_hosts` lists must be
non-empty. Additionally, the number of `worker_hosts` must be greater than or equal to the number of `master_hosts`. If
your local SSH key is protected by a passphrase, you may specify the optional `password_file` key in the `ssh_config`
with the path to a file containing the password. The `password_file` must contain a single line with the password.

Deploy all profiling applications with the following command:

```sh
creo profile deploy
```

## Starting the Profiling Benchmarks

To start the profiling benchmarks, we need to provide the `benchmark` configuration parameter.

```yaml
benchmark:
  rps: # should have at least 3 elements
    - 25
    - 75
    - 150
  iterations: 5 # should be at least 3
  benchmarks_duration: 900 # in seconds, i.e., 900s = 15min
  virtual_users: 150
  warmup_pause: 15 # in seconds
  warmup_duration: 120 # in seconds, i.e., 120s = 2min
  warmup_rps: 10
  timeout: 8000 # in milliseconds
  records: 3000000
```

Start all profiling benchmarks:

```sh
creo profile benchmark
```

## Downloading and Aggregating Benchmark Measurements

After all profiling benchmarks have finished, pull and aggregate all benchmark results.
This will create the `utilization.yml` file for every handler function.

```sh
creo profile pull
```

```sh
creo profile aggregate
```

This enables generating microservice applications for the specified language.
