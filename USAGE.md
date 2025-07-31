# 使用指南

## 快速开始

### 1. 设置环境变量

```bash
export WECHAT_APP_ID="你的微信应用ID"
export WECHAT_APP_SECRET="你的微信应用密钥"
export WECHAT_REDIRECT_URI="http://localhost:3000/callback"
```

或者创建 `.env` 文件：

```bash
cp .env.example .env
# 编辑 .env 文件填入实际配置
```

### 2. 运行应用

```bash
# 使用启动脚本（推荐）
./start.sh

# 或者直接运行
cargo run
```

### 3. 访问应用

打开浏览器访问 http://localhost:3000

## 微信开放平台配置

1. 登录 [微信开放平台](https://open.weixin.qq.com/)
2. 创建网站应用
3. 获取 AppID 和 AppSecret
4. 设置授权回调域名为你的域名（开发环境可以是 localhost:3000）

## 项目结构说明

```
├── src/
│   ├── main.rs          # Web应用主程序
│   ├── lib.rs           # 库入口和测试
│   ├── wechat_auth.rs   # 微信认证核心库
│   └── config.rs        # 配置管理
├── templates/           # HTML模板
│   ├── base.html       # 基础布局
│   ├── index.html      # 首页
│   ├── profile.html    # 用户信息页
│   └── error.html      # 错误页面
├── Cargo.toml          # 项目依赖
├── Dockerfile          # Docker部署文件
├── start.sh            # 启动脚本
└── README.md           # 项目文档
```

## API使用示例

### 基本用法

```rust
use wechat_auth::{WeChatAuth, WeChatConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WeChatConfig {
        app_id: "your_app_id".to_string(),
        app_secret: "your_app_secret".to_string(),
        redirect_uri: "http://localhost:3000/callback".to_string(),
    };
    
    let auth = WeChatAuth::new(config);
    
    // 1. 生成授权URL
    let state = WeChatAuth::generate_state();
    let auth_url = auth.get_auth_url(&state);
    println!("请访问: {}", auth_url);
    
    // 2. 用户授权后，使用返回的code获取token
    let code = "用户授权后返回的code";
    let auth_response = auth.get_access_token(code).await?;
    
    // 3. 获取用户信息
    let user_info = auth.get_user_info(
        &auth_response.access_token, 
        &auth_response.openid
    ).await?;
    
    println!("用户昵称: {}", user_info.nickname);
    println!("用户OpenID: {}", user_info.openid);
    
    Ok(())
}
```

### 错误处理

```rust
match auth.get_access_token(code).await {
    Ok(response) => {
        println!("获取token成功: {}", response.access_token);
    }
    Err(e) => {
        eprintln!("获取token失败: {}", e);
    }
}
```

## 部署

### Docker部署

```bash
# 构建镜像
docker build -t wechat-auth-app .

# 运行容器
docker run -p 3000:3000 \
  -e WECHAT_APP_ID="your_app_id" \
  -e WECHAT_APP_SECRET="your_app_secret" \
  -e WECHAT_REDIRECT_URI="http://your-domain.com/callback" \
  wechat-auth-app
```

### 生产环境注意事项

1. **HTTPS**: 生产环境必须使用HTTPS
2. **域名配置**: 确保微信开放平台配置的回调域名与实际域名一致
3. **环境变量**: 使用安全的方式管理敏感信息
4. **日志**: 添加适当的日志记录
5. **监控**: 添加健康检查和监控

## 常见问题

### Q: 授权时提示"redirect_uri参数错误"
A: 检查微信开放平台的授权回调域名设置是否正确

### Q: 获取用户信息失败
A: 确保scope参数设置为"snsapi_userinfo"，并且用户已经授权

### Q: 本地开发如何测试
A: 可以使用ngrok等工具将本地服务暴露到公网，然后在微信开放平台配置对应的域名

## 扩展功能

你可以基于这个库扩展更多功能：

- 用户会话管理
- 数据库集成
- 缓存支持
- 日志记录
- 监控和统计
- 多语言支持