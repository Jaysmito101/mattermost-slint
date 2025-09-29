use super::types::*;

#[derive(Debug, Clone)]
pub struct WebApi {
    pub(super) commands: (
        flume::Sender<WebApiCommand>,
        flume::Receiver<WebApiCommand>,
    ),
}

impl Default for WebApi {
    fn default() -> Self {
        Self::new()
    }
}

impl WebApi {
    pub fn new() -> Self {
        let commands = flume::unbounded();
        Self { commands }
    }

    fn send_command(&self, command: WebApiCommand) -> Result<(), crate::Error> {
        self.commands
            .0
            .send(command)
            .map_err(|_| crate::Error::ChannelError)
    }

    pub fn set_config(
        &self,
        base_url: &str,
        api_version: &str,
        callback: impl FnOnce() + 'static + Send,
    ) -> Result<(), crate::Error> {
        self.send_command(WebApiCommand::SetConfig(
            base_url.to_string(),
            api_version.to_string(),
            Box::new(callback),
        ))?;
        Ok(())
    }

    pub fn user_login(
        &self,
        login_data: LoginData,
        callback: impl FnOnce(Result<LoginResponse, crate::Error>) + 'static + Send,
    ) -> Result<(), crate::Error> {
        self.send_command(WebApiCommand::UserLogin(login_data, Box::new(callback)))?;
        Ok(())
    }
}