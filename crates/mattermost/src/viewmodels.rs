use std::sync::Arc;

mod loginpage;
pub use loginpage::*;
use slint::Weak;

use crate::services::ServicesApi;

#[derive(macros::Getters)]
#[allow(dead_code)]
pub struct ViewModels {
    pub loginpage: LoginPageManager,
}

pub async fn initialize(ui: Weak<crate::Main>, api: ServicesApi) -> Result<Arc<ViewModels>, crate::Error> {
    let loginpage = LoginPageManager::new(ui, api).await?;

    Ok(Arc::new(ViewModels { loginpage }))
}
