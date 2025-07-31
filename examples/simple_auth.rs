// 简单的微信认证示例
// 运行方式: cargo run --example simple_auth

use my_rust_project::wechat_auth::{WeChatAuth, WeChatConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 微信OAuth2认证示例");
    
    // 配置微信应用信息
    let config = WeChatConfig {
        app_id: std::env::var("WECHAT_APP_ID")
            .unwrap_or_else(|_| "demo_app_id".to_string()),
        app_secret: std::env::var("WECHAT_APP_SECRET")
            .unwrap_or_else(|_| "demo_app_secret".to_string()),
        redirect_uri: "http://localhost:3000/callback".to_string(),
    };
    
    let auth = WeChatAuth::new(config);
    
    // 生成状态参数（用于CSRF保护）
    let state = WeChatAuth::generate_state();
    println!("📝 生成状态参数: {}", state);
    
    // 生成授权URL
    let auth_url = auth.get_auth_url(&state);
    println!("🌐 授权URL: {}", auth_url);
    
    println!("\n📋 使用步骤:");
    println!("1. 用户访问上面的授权URL");
    println!("2. 用户在微信中授权");
    println!("3. 微信重定向到回调地址，带上code参数");
    println!("4. 使用code换取access_token");
    println!("5. 使用access_token获取用户信息");
    
    // 模拟授权流程（实际使用中，code来自微信回调）
    println!("\n⚠️  注意: 这只是一个演示，实际的code需要从微信授权回调中获取");
    
    // 状态验证示例
    let received_state = state.clone();
    if WeChatAuth::validate_state(&state, &received_state) {
        println!("✅ 状态验证通过");
    } else {
        println!("❌ 状态验证失败");
    }
    
    Ok(())
}