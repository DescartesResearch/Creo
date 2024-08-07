use super::Client;
use super::{client::get_auth_method, Config};
use super::{Error, Result};
use futures::future::try_join_all;

pub async fn establish_connections(
    config: &Config,
    hosts: impl IntoIterator<Item = &String>,
) -> Result<Vec<Client>> {
    let auth_method = get_auth_method(config).await?;
    try_join_all(hosts.into_iter().map(|host| async {
        let (host, port) = match host.split_once(":") {
            Some((host, port)) => (
                host,
                port.parse::<u16>().map_err(|_| {
                    Error::Connection(format!("{} is not a valid port for host {}", port, host))
                })?,
            ),
            None => (host.as_str(), 22),
        };
        Client::connect(host, port, &config.user_name, auth_method.clone()).await
    }))
    .await
}
