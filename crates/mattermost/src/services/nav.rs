use slint::ComponentHandle;

#[derive(Debug, Clone)]
pub enum NavigationApiCommand {
    UpdateLoader(bool, flume::Sender<()>),
}

#[derive(Debug, Clone)]
pub struct NavigationApi {
    commands: (flume::Sender<NavigationApiCommand>, flume::Receiver<NavigationApiCommand>),
}

impl NavigationApi {
    pub fn new() -> Self {
        let commands = flume::unbounded();
        Self { commands }
    }

    fn send_command(&self, command: NavigationApiCommand) -> Result<(), crate::Error> {
        self.commands.0.send(command).map_err(|_|crate::Error::ChannelError)
    }

    pub async fn update_loader(&self, show: bool) -> Result<(), crate::Error> {
        let (sender, receiver) = flume::bounded(1);
        self.send_command(NavigationApiCommand::UpdateLoader(show, sender))?;
        log::warn!("Sent update_loader command, waiting for response");
        receiver.recv_async().await.map_err(|_|crate::Error::ChannelError)?;
        log::warn!("Received response for update_loader command");
        Ok(())
    }

    pub async fn hide_loader(&self) -> Result<(), crate::Error> {
        self.update_loader(false).await
    }
}

pub struct NavigationService {
    navigation: NavigationApi,
    ui: slint::Weak<crate::Main>,
}

impl NavigationService {
    pub async fn new(ui: slint::Weak<crate::Main>, navigation: NavigationApi) -> Result<Self, crate::Error> {
        Ok(Self { navigation, ui })
    }

    pub fn start(&self) {
        let navigation = self.navigation.clone();
        let ui = self.ui.clone();

        // Could also be a std::thread::spawn?
        tokio::task::spawn(async move {
            while let Ok(command) = navigation.commands.1.recv_async().await {
                match command {
                    NavigationApiCommand::UpdateLoader(show, responder) => {
                        ui.upgrade_in_event_loop(move |ui| {
                            let store = ui.global::<crate::NavStore>();
                            store.set_currentPopup(if show { crate::CurrentPopup::Loading } else { crate::CurrentPopup::None });
                            responder.send(()).ok();
                        }).ok();
                    }
                }
            }
        });
    }

}