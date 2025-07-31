# 开发环境设置指南

## 问题
微信OAuth2要求回调地址必须是公网可访问的域名，本地localhost无法直接测试。

## 解决方案

### 方案1：使用ngrok内网穿透（推荐）

1. **安装ngrok**
```bash
# macOS
brew install ngrok

# 或者从官网下载: https://ngrok.com/
```

2. **启动本地服务**
```bash
cargo run
```

3. **在另一个终端启动ngrok**
```bash
ngrok http 3000
```

4. **配置微信开放平台**
- 将ngrok提供的公网地址（如：https://abc123.ngrok.io）配置到微信开放平台
- 回调地址设置为：https://abc123.ngrok.io/callback

5. **更新环境变量**
```bash
export WECHAT_REDIRECT_URI="https://abc123.ngrok.io/callback"
```

### 方案2：使用localtunnel

```bash
# 安装
npm install -g localtunnel

# 启动
lt --port 3000 --subdomain your-app-name
```

### 方案3：使用frp自建内网穿透

如果你有自己的服务器，可以使用frp搭建内网穿透。

## 开发环境Mock方案

为了方便开发，我们也可以创建一个Mock模式：