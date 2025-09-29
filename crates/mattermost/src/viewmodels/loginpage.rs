use slint::{ComponentHandle, Weak};

use crate::services::ServicesApi;

pub struct LoginPageManager {}

impl LoginPageManager {
    pub async fn new(ui: Weak<crate::Main>, api: ServicesApi) -> Result<Self, crate::Error> {
        let main = ui.upgrade().ok_or(crate::Error::UiUpgradeFailed)?;
        let store = main.global::<crate::LoginPageStore>();

        // let auth_service = crate::services::get::<crate::services::AuthApi>();
        // if auth_service.has_saved_credentials().await {
        //     store.set_data(aith_service.load_saved_credentials().await?);
        // }

        store.on_login_clicked(move || {
            if let Some(main) = ui.upgrade() {
                let store = main.global::<crate::LoginPageStore>();
                let data = store.get_data();
                api.navigation.update_loader(true).ok();
                
                let api_clone = api.clone();
                api.web.set_config(
                    &data.server_url,
                    "v4",
                    move || {
                        let login_data = crate::services::LoginData {
                            login_id: data.username.to_string(),
                            password: data.password.to_string(),
                            ..Default::default()
                        };
                        let api = api_clone.clone();
                        api_clone.clone().web.user_login(login_data, move |result| {
                            api.navigation.update_loader(false).ok();

                            match result {
                                Ok(response) => {
                                    log::warn!("Login successful: {:?}", response);
                                }
                                Err(err) => {
                                    log::error!("Login failed: {:?}", err);
                                }
                            }
                        }).unwrap_or_else(|err| log::error!("Failed to send login request: {:?}", err));
                    },
                ).unwrap_or_else(|err| log::error!("Failed to set config: {:?}", err));
            }
        });

        // event.subscribe(Event.LoggedIn, move |_| {
        //     navigation_service.navigate_to(crate::NavigationTarget::MainPage);
        // });

        Ok(Self {})
    }
}
