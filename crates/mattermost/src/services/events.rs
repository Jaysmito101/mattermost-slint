use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Events {
    Dummy,
}

#[derive(Clone, Debug)]
pub enum EventsData {
    Dummy,
}

pub enum EventsApiCommand {
    Subscribe(Events, Box<dyn Fn(&EventsData) + Send>),
    Post(Events, EventsData),
}

#[derive(Debug, Clone)]
pub struct EventsApi {
    commands: (
        flume::Sender<EventsApiCommand>,
        flume::Receiver<EventsApiCommand>,
    ),
}

impl EventsApi {
    pub fn new() -> Self {
        let commands = flume::unbounded();
        Self { commands }
    }

    fn send_command(&self, command: EventsApiCommand) -> Result<(), crate::Error> {
        self.commands
            .0
            .send(command)
            .map_err(|_| crate::Error::ChannelError)
    }

    pub fn subscribe(
        &self,
        event: Events,
        callback: impl Fn(&EventsData) + 'static + Send,
    ) -> Result<(), crate::Error> {
        self.send_command(EventsApiCommand::Subscribe(event, Box::new(callback)))?;
        Ok(())
    }

    pub fn post(&self, event: Events, data: EventsData) -> Result<(), crate::Error> {
        self.send_command(EventsApiCommand::Post(event, data))?;
        Ok(())
    }
}

pub struct EventsService {
    events: EventsApi,
}

impl EventsService {
    pub async fn new(events: EventsApi) -> Result<Self, crate::Error> {
        Ok(Self { events })
    }

    pub fn start(&self) {
        let events = self.events.clone();

        // Could also be a std::thread::spawn?
        tokio::task::spawn(async move {
            let mut callbacks = HashMap::<Events, Vec<Box<dyn Fn(&EventsData) + Send>>>::new();

            while let Ok(command) = events.commands.1.recv_async().await {
                match command {
                    EventsApiCommand::Subscribe(event, callback) => {
                        callbacks.entry(event).or_default().push(callback);
                    }
                    EventsApiCommand::Post(event, data) => {
                        if let Some(cbs) = callbacks.get(&event) {
                            for cb in cbs {
                                cb(&data);
                            }
                        }
                    }
                }
            }
        });
    }
}
