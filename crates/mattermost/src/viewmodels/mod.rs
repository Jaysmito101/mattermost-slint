use std::sync::Arc;

mod loginpage;
pub use loginpage::*;
use slint::Weak;

#[derive(macros::Getters)]
#[allow(dead_code)]
pub struct ViewModels {
    pub loginpage: LoginPageManager,
}

pub async fn initialize(ui: Weak<crate::Main>) -> Result<Arc<ViewModels>, crate::Error> {
    let loginpage = LoginPageManager::new(ui).await?;
    

    Ok(Arc::new(ViewModels { loginpage }))
}

