use slint::Weak;
use std::sync::Arc;

mod nav;
pub use nav::*;

mod web;
pub use web::*;

mod events;
pub use events::*;

#[derive(Debug, Clone, macros::Getters)]
pub struct ServicesApi {
    pub navigation: NavigationApi,
    pub events: EventsApi,
    pub web: WebApi,
}

impl ServicesApi {
    fn new() -> Self {
        Self {
            navigation: NavigationApi::new(),
            events: EventsApi::new(),
            web: WebApi::new(),
        }
    }
}

#[allow(dead_code)]
pub struct Services {
    navigation: NavigationService,
    events: EventsService,
    web: WebService,
    api: ServicesApi,
}

impl Services {
    pub fn api(&self) -> &ServicesApi {
        &self.api
    }
}

pub async fn initialize(ui: Weak<crate::Main>) -> Result<Arc<Services>, crate::Error> {
    let api = ServicesApi::new();

    let navigation = api.navigation.clone().start_service(ui)?;
    let events = api.events.clone().start_service()?;
    let web = api.web.clone().start_service()?;

    Ok(Arc::new(Services { navigation, events, web, api }))
}
