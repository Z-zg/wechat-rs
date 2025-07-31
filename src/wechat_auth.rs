use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatConfig {
    pub app_id: String,
    pub app_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatAuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatUserInfo {
    pub openid: String,
    pub nickname: String,
    pub sex: i32,
    pub province: String,
    pub city: String,
    pub country: String,
    pub headimgurl: String,
    pub privilege: Vec<String>,
    pub unionid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeChatError {
    pub errcode: i32,
    pub errmsg: String,
}

pub struct WeChatAuth {
    config: WeChatConfig,
    client: reqwest::Client,
}

impl WeChatAuth {
    pub fn new(config: WeChatConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Generate WeChat OAuth2 authorization URL
    pub fn get_auth_url(&self, state: &str) -> String {
        format!(
            "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope=snsapi_userinfo&state={}#wechat_redirect",
            self.config.app_id,
            urlencoding::encode(&self.config.redirect_uri),
            state
        )
    }

    /// Exchange authorization code for access token
    pub async fn get_access_token(&self, code: &str) -> Result<WeChatAuthResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",
            self.config.app_id,
            self.config.app_secret,
            code
        );

        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        // Check if response contains error
        if text.contains("errcode") {
            let error: WeChatError = serde_json::from_str(&text)?;
            return Err(format!("WeChat API Error: {} - {}", error.errcode, error.errmsg).into());
        }

        let auth_response: WeChatAuthResponse = serde_json::from_str(&text)?;
        Ok(auth_response)
    }

    /// Get user information using access token
    pub async fn get_user_info(&self, access_token: &str, openid: &str) -> Result<WeChatUserInfo, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}&lang=zh_CN",
            access_token,
            openid
        );

        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        // Check if response contains error
        if text.contains("errcode") {
            let error: WeChatError = serde_json::from_str(&text)?;
            return Err(format!("WeChat API Error: {} - {}", error.errcode, error.errmsg).into());
        }

        let user_info: WeChatUserInfo = serde_json::from_str(&text)?;
        Ok(user_info)
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<WeChatAuthResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.weixin.qq.com/sns/oauth2/refresh_token?appid={}&grant_type=refresh_token&refresh_token={}",
            self.config.app_id,
            refresh_token
        );

        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        if text.contains("errcode") {
            let error: WeChatError = serde_json::from_str(&text)?;
            return Err(format!("WeChat API Error: {} - {}", error.errcode, error.errmsg).into());
        }

        let auth_response: WeChatAuthResponse = serde_json::from_str(&text)?;
        Ok(auth_response)
    }

    /// Generate state parameter for CSRF protection
    pub fn generate_state() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Validate state parameter
    pub fn validate_state(expected: &str, received: &str) -> bool {
        expected == received
    }
}