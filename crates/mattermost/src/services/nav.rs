use slint::ComponentHandle;

pub enum NavigationApiCommand {
    UpdateLoader(bool),
}

#[derive(Debug, Clone)]
pub struct NavigationApi {
    commands: (
        flume::Sender<NavigationApiCommand>,
        flume::Receiver<NavigationApiCommand>,
    ),
}

pub struct NavigationService {
    pub navigation: NavigationApi,
    pub ui: slint::Weak<crate::Main>,
}

impl NavigationApi {
    pub fn new() -> Self {
        let commands = flume::unbounded();
        Self { commands }
    }

    fn send_command(&self, command: NavigationApiCommand) -> Result<(), crate::Error> {
        self.commands
            .0
            .send(command)
            .map_err(|_| crate::Error::ChannelError)
    }

    pub fn update_loader(
        &self,
        show: bool,
    ) -> Result<(), crate::Error> {
        self.send_command(NavigationApiCommand::UpdateLoader(
            show,
        ))?;
        Ok(())
    }

    pub fn start_service(
        self,
        ui: slint::Weak<crate::Main>,
    ) -> Result<NavigationService, crate::Error> {
        let navigation = self.clone();

        let navigation_service = NavigationService {
            navigation: self,
            ui: ui.clone(),
        };
        // Could also be a std::thread::spawn?
        tokio::task::spawn(async move {
            while let Ok(command) = navigation.commands.1.recv_async().await {
                match command {
                    NavigationApiCommand::UpdateLoader(show) => {
                        ui.upgrade_in_event_loop(move |ui| {
                            let store = ui.global::<crate::NavStore>();
                            store.set_currentPopup(if show {
                                crate::CurrentPopup::Loading
                            } else {
                                crate::CurrentPopup::None
                            });
                        })
                        .ok();
                    }
                }
            }
        });
        Ok(navigation_service)
    }
}
