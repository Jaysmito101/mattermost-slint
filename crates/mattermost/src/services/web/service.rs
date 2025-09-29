use super::types::*;
use super::api::WebApi;

pub struct WebService {
    pub web: WebApi,
}

impl WebApi {
    pub fn start_service(self) -> Result<WebService, crate::Error> {
        let web = self.clone();
        let web_service = WebService { web: self };

        tokio::task::spawn(async move {
            let mut config = WebConfig::default();

            while let Ok(command) = web.commands.1.recv_async().await {
                match command {
                    WebApiCommand::SetConfig(base_url, api_version, callback) => {
                        config.base_url = base_url;
                        config.api_version = api_version;
                        callback();
                    }
                    WebApiCommand::UserLogin(login_data, callback) => {
                        let response = Self::mock_login_response(&login_data).await;
                        callback(Ok(response));
                    }
                }
            }
        });

        Ok(web_service)
    }

    async fn mock_login_response(login_data: &LoginData) -> LoginResponse {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Simulate network delay
        let mock_user = User {
            id: "mock_user_id_12345".to_string(),
            create_at: 1234567890000,
            update_at: 1234567890000,
            delete_at: 0,
            username: login_data.login_id.split('@').next().unwrap_or("user").to_string(),
            first_name: Some("Mock".to_string()),
            last_name: Some("User".to_string()),
            nickname: None,
            email: login_data.login_id.clone(),
            email_verified: true,
            auth_service: None,
            roles: "system_user".to_string(),
            locale: "en".to_string(),
            notify_props: None,
            props: None,
            last_password_update: Some(1234567890000),
            last_picture_update: Some(1234567890000),
            failed_attempts: 0,
            mfa_active: false,
            timezone: None,
            terms_of_service_id: None,
            terms_of_service_create_at: None,
        };

        LoginResponse {
            user: mock_user,
            token: "mock_session_token_abcdef123456789".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct WebConfig {
    base_url: String,
    api_version: String,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8065".to_string(),
            api_version: "v4".to_string(),
        }
    }
}