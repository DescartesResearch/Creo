pub mod cli;
mod commands;
pub mod config;
mod error;
mod io;
mod util;

pub use error::{Error, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn entrypoint(args: cli::Args, stdout: &mut impl std::io::Write) -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    if args.version {
        writeln!(stdout, "Creo v{VERSION}")?;
        return Ok(());
    }

    let command = match args.command {
        Some(command) => command,
        None => {
            log::error!("Error: missing command! For usage help try using the `--help` flag!");
            std::process::exit(1);
        }
    };
    let root = std::env::current_dir().map_err(|err| Error::with_log("failed to locate the current working directory! Make sure the executable has sufficient permissions to access the directory!".into(), err))?;
    let output = args.output.unwrap_or(creo_lib::OUTPUT_DIR.into());
    match command {
        cli::Commands::Generate(args) => {
            let config = io::parse_config::<config::generate::Config>(&args.config)?;
            let result = commands::generate::generate(&config, &root, &output);
            match result {
                Ok(_) => log::info!("Successfully generated application!"),
                Err(err) => {
                    log::error!("{}", err);
                    let app_dir = root.join(output).join(config.app_name.as_ref());
                    util::cleanup_dir(&app_dir);
                }
            }
        }
        cli::Commands::Profile(profile) => match profile.command {
            cli::profile::ProfileSubCommands::Generate(args) => {
                let config = io::parse_config::<cli::profile::generate::Config>(&args.config)?;
                let result = commands::profile::generate(&config, &root);
                match result {
                    Ok(_) => log::info!("Successfully generated profiling services!"),
                    Err(err) => {
                        log::error!("{}", err);
                        let app_dir = root.join(creo_lib::PROFILE_DIR).join(
                            commands::profile::generate_profile_app_dir_name(&config.language),
                        );
                        util::cleanup_dir(&app_dir);
                    }
                }
            }
            cli::profile::ProfileSubCommands::Deploy(args) => {
                let config = io::parse_config::<cli::profile::deploy::Config>(&args.config)?;
                let result = commands::profile::invoke(&config.ssh_config, config.app_name).await;
                match result {
                    Ok(_) => log::info!("Successfully deployed profiling services!"),
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
            cli::profile::ProfileSubCommands::Benchmark(args) => {
                let config = io::parse_config::<cli::profile::benchmark::Config>(&args.config)?;
                let result = commands::profile::benchmark(&config).await;
                match result {
                    Ok(_) => log::info!("Successfully started benchmarks for profiling services!"),
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
            cli::profile::ProfileSubCommands::Pull(args) => {
                let config = io::parse_config::<cli::profile::pull::Config>(&args.config)?;
                let result = commands::profile::pull(&config).await;
                match result {
                    Ok(_) => log::info!("Successfully pulled benchmarking results!"),
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
            cli::profile::ProfileSubCommands::Aggregate(args) => {
                let config = io::parse_config::<cli::profile::aggregate::Config>(&args.config)?;
                let result = commands::profile::aggregate(&config).await;
                match result {
                    Ok(_) => log::info!("Successfully aggregated benchmarking results!"),
                    Err(err) => {
                        log::error!("{}", err);
                    }
                }
            }
        },
        cli::Commands::Deploy(args) => {
            let config = io::parse_config::<cli::deploy::Config>(&args.config)?;
            let result = commands::deploy::invoke(&config.ssh, config.application, output).await;
            match result {
                Ok(_) => log::info!("Deployment finished successfully!"),
                Err(err) => {
                    log::error!("{}", err)
                }
            }
        }
        cli::Commands::Benchmark(args) => {
            let config = io::parse_config::<cli::benchmark::Config>(&args.config)?;
            let result =
                commands::benchmark::invoke(&config.ssh, config.application, &config.benchmark)
                    .await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    log::error!("{}", err)
                }
            }
        }
        cli::Commands::Download(args) => {
            let config = io::parse_config::<cli::download::Config>(&args.config)?;
            let result = commands::download::invoke(&config.ssh, config.application, output).await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    log::error!("{}", err)
                }
            }
        }
    }

    Ok(())
}
