// 微信服务器功能示例
// 运行方式: cargo run --example wechat_server_example

use my_rust_project::wechat_server::{WeChatServerConfig, WeChatMessage};

fn main() {
    println!("🔧 微信服务器功能示例");
    
    // 创建微信服务器配置
    let server_config = WeChatServerConfig::new(
        "your_token".to_string(),
        "your_app_id".to_string(),
        "your_app_secret".to_string(),
    );
    
    // 示例：验证签名
    let signature = "example_signature";
    let timestamp = "1234567890";
    let nonce = "example_nonce";
    
    println!("🔐 签名验证示例:");
    println!("   Signature: {}", signature);
    println!("   Timestamp: {}", timestamp);
    println!("   Nonce: {}", nonce);
    
    if server_config.verify_signature(signature, timestamp, nonce) {
        println!("✅ 签名验证通过");
    } else {
        println!("❌ 签名验证失败");
    }
    
    // 示例：处理文本消息
    let mock_message = WeChatMessage {
        to_user_name: "公众号".to_string(),
        from_user_name: "用户openid".to_string(),
        create_time: 1234567890,
        msg_type: "text".to_string(),
        content: Some("你好".to_string()),
        msg_id: Some(123456),
        event: None,
        event_key: None,
    };
    
    println!("\n📝 消息处理示例:");
    println!("   用户消息: {:?}", mock_message.content);
    
    let response = server_config.handle_message(mock_message);
    println!("   服务器回复: {}", response);
    
    // 示例：处理关注事件
    let subscribe_event = WeChatMessage {
        to_user_name: "公众号".to_string(),
        from_user_name: "用户openid".to_string(),
        create_time: 1234567890,
        msg_type: "event".to_string(),
        content: None,
        msg_id: None,
        event: Some("subscribe".to_string()),
        event_key: None,
    };
    
    println!("\n🎉 关注事件处理示例:");
    let welcome_response = server_config.handle_message(subscribe_event);
    println!("   欢迎消息: {}", welcome_response);
    
    println!("\n📋 配置说明:");
    println!("1. 在微信公众平台配置服务器URL: https://yourdomain.com/wechat");
    println!("2. 设置Token与代码中的token保持一致");
    println!("3. 启动服务器后点击微信公众平台的'提交'按钮验证");
    println!("4. 用户发送消息后会收到自动回复");
}