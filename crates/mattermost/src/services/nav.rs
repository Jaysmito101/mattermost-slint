use slint::ComponentHandle;

pub enum NavigationApiCommand {
    UpdateLoader(bool, Option<Box<dyn FnOnce(&mut crate::Main) + Send>>),
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

    pub fn update_loader(&self, show: bool, callback: Option<impl 'static + FnOnce(&mut crate::Main) + Send>) -> Result<(), crate::Error>
    {
        self.send_command(NavigationApiCommand::UpdateLoader(show, callback.map(|cb| Box::new(cb) as Box<dyn FnOnce(&mut crate::Main) + Send>)))?;
        Ok(())
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
                        ui.upgrade_in_event_loop(move |mut ui| {
                            let store = ui.global::<crate::NavStore>();
                            store.set_currentPopup(if show { crate::CurrentPopup::Loading } else { crate::CurrentPopup::None });
                            if let Some(cb) = responder {
                                cb(&mut ui);
                            }
                        }).ok();
                    }
                }
            }
        });
    }

}