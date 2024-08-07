use std::{
    io::{BufRead, IsTerminal},
    net::IpAddr,
};

use super::{Error, Result};

pub struct Client {
    client: async_ssh2_tokio::client::Client,
    sftp_session: russh_sftp::client::SftpSession,
}

impl Client {
    pub async fn connect(
        host: impl AsRef<str>,
        port: u16,
        username: impl AsRef<str>,
        auth_method: async_ssh2_tokio::client::AuthMethod,
    ) -> Result<Self> {
        let client = async_ssh2_tokio::client::Client::connect(
            (host.as_ref(), port),
            username.as_ref(),
            auth_method,
            async_ssh2_tokio::client::ServerCheckMethod::NoCheck,
        )
        .await
        .map_err(|err| Error::Connection(err.to_string()))?;
        let channel = client
            .get_channel()
            .await
            .map_err(|err| Error::Connection(err.to_string()))?;
        channel
            .request_subsystem(true, "sftp")
            .await
            .map_err(|err| Error::Connection(err.to_string()))?;
        let sftp_session = russh_sftp::client::SftpSession::new(channel.into_stream())
            .await
            .map_err(|err| Error::Connection(err.to_string()))?;

        Ok(Self {
            client,
            sftp_session,
        })
    }

    pub async fn execute(
        &self,
        command: impl AsRef<str>,
    ) -> Result<async_ssh2_tokio::client::CommandExecutedResult> {
        let result = self.client.execute(command.as_ref()).await?;
        Ok(result)
    }

    pub async fn canonicalize(&self, path: impl Into<String>) -> Result<String> {
        let result = self.sftp_session.canonicalize(path).await?;
        Ok(result)
    }

    pub fn get_connection_ip(&self) -> IpAddr {
        self.client.get_connection_address().ip()
    }

    pub async fn try_exists(&self, path: impl Into<String>) -> Result<bool> {
        Ok(self.sftp_session.try_exists(path).await?)
    }

    pub async fn read(&self, path: impl Into<String>) -> Result<Vec<u8>> {
        Ok(self.sftp_session.read(path).await?)
    }

    pub async fn read_dir(
        &self,
        path: impl Into<String>,
    ) -> Result<russh_sftp::client::fs::ReadDir> {
        Ok(self.sftp_session.read_dir(path).await?)
    }

    pub async fn create_dir(&self, path: impl Into<String>) -> Result<()> {
        Ok(self.sftp_session.create_dir(path).await?)
    }

    pub async fn create(&self, path: impl Into<String>) -> Result<russh_sftp::client::fs::File> {
        Ok(self.sftp_session.create(path).await?)
    }

    pub async fn open(&self, path: impl Into<String>) -> Result<russh_sftp::client::fs::File> {
        Ok(self.sftp_session.open(path).await?)
    }
}

pub async fn get_auth_method(config: &super::Config) -> Result<async_ssh2_tokio::AuthMethod> {
    let file_pw = if let Some(ref pw_file) = config.password_file {
        Some(
            tokio::fs::read_to_string(pw_file)
                .await
                .map_err(|err| {
                    Error::AuthMethod(format!(
                        "failed to read password file for path {}!\n\tReason: {}",
                        pw_file,
                        err
                    ))
                })?
                .trim_end()
                .into(),
        )
    } else {
        None
    };
    let stdin_pw = read_pw_from_stdin()?;
    if let Some(ref key_file) = config.key_file {
        return Ok(async_ssh2_tokio::AuthMethod::with_key_file(
            key_file,
            stdin_pw.or(file_pw).as_deref(),
        ));
    }
    let password = stdin_pw.or(file_pw).ok_or_else(|| {
        Error::AuthMethod("missing authentication method! Either provide a key file or password".to_string())
    })?;

    Ok(async_ssh2_tokio::AuthMethod::with_password(&password))
}

pub fn read_pw_from_stdin() -> Result<Option<String>> {
    let stdin = std::io::stdin();
    if stdin.lock().is_terminal() {
        return Ok(None);
    }
    match stdin.lock().lines().next() {
        None => Ok(None),
        Some(line) => match line {
            Ok(pw) => Ok(Some(pw)),
            Err(err) => Err(Error::AuthMethod(format!(
                "could not read line from stdin!\n\t Reason: {}",
                err
            ))),
        },
    }
}
