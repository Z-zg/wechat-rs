pub mod wechat_auth;
pub mod wechat_server;

pub use wechat_auth::{WeChatAuth, WeChatConfig, WeChatUserInfo, WeChatAuthResponse};
pub use wechat_server::{WeChatServerConfig, WeChatMessage, WeChatVerifyQuery, WeChatMessageQuery};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_state() {
        let state1 = WeChatAuth::generate_state();
        let state2 = WeChatAuth::generate_state();
        
        assert_ne!(state1, state2);
        assert!(!state1.is_empty());
        assert!(!state2.is_empty());
    }

    #[test]
    fn test_validate_state() {
        let state = "test-state-123";
        assert!(WeChatAuth::validate_state(state, state));
        assert!(!WeChatAuth::validate_state(state, "different-state"));
    }

    #[test]
    fn test_auth_url_generation() {
        let config = WeChatConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
        };
        
        let auth = WeChatAuth::new(config);
        let url = auth.get_auth_url("test_state");
        
        assert!(url.contains("test_app_id"));
        assert!(url.contains("test_state"));
        assert!(url.contains("snsapi_userinfo"));
    }
}