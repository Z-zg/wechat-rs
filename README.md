# WeChat OAuth2 认证库和Web应用

这是一个用Rust编写的微信OAuth2认证库和传统Web应用示例。

## 功能特性

- 🔐 完整的微信OAuth2认证流程
- 🌐 传统Web应用界面（使用Askama模板）
- 🛡️ CSRF保护（状态参数验证）
- 📱 响应式设计
- 🚀 高性能异步处理（基于Tokio和Axum）

## 项目结构

```
├── src/
│   ├── main.rs           # Web应用主程序
│   └── wechat_auth.rs    # 微信认证库
├── templates/            # HTML模板
│   ├── base.html        # 基础模板
│   ├── index.html       # 首页
│   ├── profile.html     # 用户信息页
│   └── error.html       # 错误页面
├── Cargo.toml           # 项目配置
└── .env.example         # 环境变量示例
```

## 快速开始

### 1. 克隆项目并安装依赖

```bash
cargo build
```

### 2. 配置微信应用

1. 在微信开放平台创建应用
2. 复制 `.env.example` 为 `.env`
3. 填入你的微信应用配置：

```bash
cp .env.example .env
# 编辑 .env 文件，填入实际的配置信息
```

### 3. 运行应用

```bash
# 设置环境变量
export WECHAT_APP_ID="你的微信应用ID"
export WECHAT_APP_SECRET="你的微信应用密钥"
export WECHAT_REDIRECT_URI="http://localhost:3000/callback"

# 启动服务器
cargo run
```

访问 http://localhost:3000 开始使用。

## API文档

### WeChatAuth 库

```rust
use wechat_auth::{WeChatAuth, WeChatConfig};

// 创建配置
let config = WeChatConfig {
    app_id: "your_app_id".to_string(),
    app_secret: "your_app_secret".to_string(),
    redirect_uri: "http://localhost:3000/callback".to_string(),
};

// 初始化认证客户端
let auth = WeChatAuth::new(config);

// 生成授权URL
let state = WeChatAuth::generate_state();
let auth_url = auth.get_auth_url(&state);

// 获取访问令牌
let auth_response = auth.get_access_token("authorization_code").await?;

// 获取用户信息
let user_info = auth.get_user_info(&auth_response.access_token, &auth_response.openid).await?;
```

### 主要结构体

#### WeChatConfig
```rust
pub struct WeChatConfig {
    pub app_id: String,
    pub app_secret: String,
    pub redirect_uri: String,
}
```

#### WeChatUserInfo
```rust
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
```

## 部署

### Docker部署

创建 `Dockerfile`：

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my-rust-project /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
```

构建和运行：

```bash
docker build -t wechat-auth-app .
docker run -p 3000:3000 \
  -e WECHAT_APP_ID="your_app_id" \
  -e WECHAT_APP_SECRET="your_app_secret" \
  -e WECHAT_REDIRECT_URI="http://your-domain.com/callback" \
  wechat-auth-app
```

## 安全注意事项

1. **环境变量**: 永远不要将敏感信息硬编码到代码中
2. **HTTPS**: 生产环境必须使用HTTPS
3. **状态验证**: 实现了CSRF保护，但生产环境建议使用更强的会话管理
4. **错误处理**: 避免在错误信息中泄露敏感信息

## 开发

### 添加新功能

1. 在 `src/wechat_auth.rs` 中添加新的API方法
2. 在 `src/main.rs` 中添加新的路由处理
3. 在 `templates/` 中添加新的模板

### 测试

```bash
cargo test
```

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request！