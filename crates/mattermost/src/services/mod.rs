use slint::Weak;
use std::sync::{Arc, LazyLock};

mod nav;
pub use nav::*;

mod web;
pub use web::*;

mod events;
pub use events::*;

#[derive(Debug, Clone, macros::Getters)]
pub struct ServicesApi {
    navigation: NavigationApi,
    web: WebApi,
}

impl ServicesApi {
    fn new() -> Self {
        Self {
            navigation: NavigationApi::new(),
            web: WebApi::new(),
        }
    }
}

static SERVICES: LazyLock<Arc<ServicesApi>> = LazyLock::new(|| Arc::new(ServicesApi::new()));

pub(crate) fn get<T>() -> &'static T
where T: 'static + TraitServicesApi::Gettable, ServicesApi: TraitServicesApi::GetterTrait<T>
{
    SERVICES.get()
}

#[allow(dead_code)]
pub struct Services {
    navigation: NavigationService,
}

pub async fn initialize(ui: Weak<crate::Main>) -> Result<Arc<Services>, crate::Error> {
    let navigation = NavigationService::new(ui, get::<NavigationApi>().clone()).await?;
    navigation.start();


    Ok(Arc::new(Services {
        navigation
    }))
}
