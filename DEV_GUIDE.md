# 开发环境指南

## 问题说明

在本地开发环境中，微信OAuth2有以下限制：
- 回调地址必须是公网可访问的域名
- localhost无法直接接收微信的回调
- 需要HTTPS协议（生产环境）

## 解决方案

### 方案1：开发模式（推荐用于本地开发）

使用内置的Mock模式，无需真实的微信回调：

```bash
# 启动开发模式
./start-dev.sh

# 或者手动设置环境变量
export DEV_MODE=true
cargo run
```

**开发模式特性：**
- ✅ 无需配置真实的微信应用
- ✅ 使用Mock用户数据
- ✅ 完整的登录流程测试
- ✅ 适合UI和逻辑开发

### 方案2：内网穿透（用于真实微信测试）

#### 使用ngrok

1. **安装ngrok**
```bash
# macOS
brew install ngrok

# 注册并获取authtoken: https://ngrok.com/
ngrok config add-authtoken YOUR_AUTHTOKEN
```

2. **启动应用**
```bash
cargo run
```

3. **启动ngrok**
```bash
# 在另一个终端
ngrok http 3000
```

4. **配置微信开放平台**
- 复制ngrok提供的HTTPS地址（如：https://abc123.ngrok.io）
- 在微信开放平台设置回调域名
- 更新环境变量：
```bash
export WECHAT_REDIRECT_URI="https://abc123.ngrok.io/callback"
```

#### 使用localtunnel

```bash
# 安装
npm install -g localtunnel

# 启动
lt --port 3000 --subdomain your-app-name

# 访问: https://your-app-name.loca.lt
```

### 方案3：使用测试服务器

如果你有自己的服务器，可以：

1. 部署应用到服务器
2. 配置域名和SSL证书
3. 在微信开放平台配置真实域名

## 开发流程建议

### 阶段1：本地开发（使用开发模式）
```bash
# 启动开发模式
export DEV_MODE=true
cargo run

# 访问 http://localhost:3000
# 测试完整的登录流程（使用Mock数据）
```

### 阶段2：集成测试（使用内网穿透）
```bash
# 启动正常模式
export DEV_MODE=false
export WECHAT_APP_ID="your_real_app_id"
export WECHAT_APP_SECRET="your_real_secret"
cargo run

# 在另一个终端启动ngrok
ngrok http 3000

# 配置微信开放平台使用ngrok地址
# 测试真实的微信登录流程
```

### 阶段3：生产部署
```bash
# 部署到服务器
# 配置HTTPS
# 使用真实域名
```

## 环境变量配置

### 开发模式
```bash
DEV_MODE=true
WECHAT_APP_ID=dev_app_id          # 可以是任意值
WECHAT_APP_SECRET=dev_secret      # 可以是任意值
```

### 生产模式
```bash
DEV_MODE=false
WECHAT_APP_ID=your_real_app_id
WECHAT_APP_SECRET=your_real_secret
WECHAT_REDIRECT_URI=https://yourdomain.com/callback
```

## Mock数据说明

开发模式使用的Mock用户数据：
```json
{
  "openid": "mock_openid_123456",
  "nickname": "开发测试用户",
  "sex": 1,
  "province": "北京",
  "city": "北京", 
  "country": "中国",
  "headimgurl": "https://via.placeholder.com/80x80?text=Mock",
  "unionid": "mock_unionid_789"
}
```

## 常见问题

**Q: 开发模式下能测试什么？**
A: 可以测试完整的登录流程、UI界面、错误处理等，除了真实的微信API调用。

**Q: 如何切换到生产模式？**
A: 设置 `DEV_MODE=false` 并配置真实的微信应用信息。

**Q: ngrok免费版有什么限制？**
A: 每次启动URL会变化，需要重新配置微信开放平台。付费版可以使用固定域名。

**Q: 可以同时支持开发和生产模式吗？**
A: 是的，通过环境变量 `DEV_MODE` 来控制，代码会自动适配。