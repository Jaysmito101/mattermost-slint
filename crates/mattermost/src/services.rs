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
    navigation: NavigationApi,
    events: EventsApi,
    web: WebApi,
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
}

pub async fn initialize(ui: Weak<crate::Main>) -> Result<Arc<Services>, crate::Error> {
    let service_api = ServicesApi::new();
    let navigation = service_api.navigation.clone().start_service(ui)?;
    let events = service_api.events.clone().start_service()?;

    Ok(Arc::new(Services { navigation, events }))
}
