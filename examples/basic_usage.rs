use my_rust_project::{WeChatAuth, WeChatConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建微信配置
    let config = WeChatConfig {
        app_id: "your_app_id".to_string(),
        app_secret: "your_app_secret".to_string(),
        redirect_uri: "http://localhost:3000/callback".to_string(),
    };

    // 初始化微信认证客户端
    let wechat_auth = WeChatAuth::new(config);

    // 生成授权URL
    let state = WeChatAuth::generate_state();
    let auth_url = wechat_auth.get_auth_url(&state);
    
    println!("授权URL: {}", auth_url);
    println!("状态参数: {}", state);

    // 模拟获取授权码后的流程
    // let auth_response = wechat_auth.get_access_token("authorization_code").await?;
    // let user_info = wechat_auth.get_user_info(&auth_response.access_token, &auth_response.openid).await?;
    // println!("用户信息: {:?}", user_info);

    Ok(())
}