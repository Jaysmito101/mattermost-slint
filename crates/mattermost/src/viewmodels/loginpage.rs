use slint::{Weak, ComponentHandle};

pub struct LoginPageManager {}

impl LoginPageManager {
    pub async fn new(ui: Weak<crate::Main>) -> Result<Self, crate::Error> {

        let main = ui.upgrade().ok_or(crate::Error::UiUpgradeFailed)?;
        let store = main.global::<crate::LoginPageStore>();

        // let auth_service = crate::services::get::<crate::services::AuthApi>();
        // if auth_service.has_saved_credentials().await {
        //     store.set_data(aith_service.load_saved_credentials().await?);
        // }

        
        // store.on_login_clicked(move || {
        //     let store = main.global::<crate::LoginPageStore>();
        //     let nav_store = main.global::<crate::NavStore>();
        //     let data = store.get_data();
            
        //     let auth = crate::services::get::<crate::services::AuthApi>();
        //     auth.login(data);
        // });

        // event.subscribe(Event.LoggedIn, move |_| {
        //     navigation_service.navigate_to(crate::NavigationTarget::MainPage);
        // });

        Ok(Self {})
    }
}
