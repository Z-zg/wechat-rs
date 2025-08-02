use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub wechat_app_id: String,
    pub wechat_app_secret: String,
    pub wechat_redirect_uri: String,
    pub wechat_token: String,
    pub server_host: String,
    pub server_port: u16,
    pub dev_mode: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let wechat_app_id = env::var("WECHAT_APP_ID")
            .map_err(|_| "WECHAT_APP_ID environment variable is required".to_string())?;
        
        let wechat_app_secret = env::var("WECHAT_APP_SECRET")
            .map_err(|_| "WECHAT_APP_SECRET environment variable is required".to_string())?;
        
        let wechat_redirect_uri = env::var("WECHAT_REDIRECT_URI")
            .unwrap_or_else(|_| "http://localhost:3000/callback".to_string());
        
        let wechat_token = env::var("WECHAT_TOKEN")
            .unwrap_or_else(|_| "your_wechat_token".to_string());
        
        let server_host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "Invalid SERVER_PORT value".to_string())?;

        let dev_mode = env::var("DEV_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        Ok(AppConfig {
            wechat_app_id,
            wechat_app_secret,
            wechat_redirect_uri,
            wechat_token,
            server_host,
            server_port,
            dev_mode,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.wechat_app_id.is_empty() {
            return Err("WeChat App ID cannot be empty".to_string());
        }
        
        if self.wechat_app_secret.is_empty() {
            return Err("WeChat App Secret cannot be empty".to_string());
        }
        
        if !self.wechat_redirect_uri.starts_with("http") {
            return Err("WeChat Redirect URI must be a valid HTTP(S) URL".to_string());
        }
        
        Ok(())
    }
}