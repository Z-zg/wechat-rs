# 微信服务器配置指南

## 概述

除了OAuth2登录功能，本项目还支持微信公众号/小程序的服务器配置，可以接收和处理微信推送的消息和事件。

## 功能特性

- ✅ 服务器URL验证
- ✅ 消息接收和自动回复
- ✅ 事件处理（关注/取消关注）
- ✅ 菜单点击事件处理
- ✅ 签名验证保证安全性

## 配置步骤

### 1. 环境变量配置

在 `.env` 文件中添加：

```bash
# 微信服务器配置
WECHAT_TOKEN=your_server_token_here
WECHAT_APP_ID=your_app_id
WECHAT_APP_SECRET=your_app_secret
```

### 2. 微信公众平台配置

1. 登录 [微信公众平台](https://mp.weixin.qq.com/)
2. 进入"开发" -> "基本配置"
3. 配置服务器信息：
   - **URL**: `https://yourdomain.com/wechat`
   - **Token**: 与环境变量 `WECHAT_TOKEN` 保持一致
   - **EncodingAESKey**: 可选，用于消息加密

### 3. 启动应用

```bash
# 设置环境变量
export WECHAT_TOKEN="your_token_here"
export WECHAT_APP_ID="your_app_id"
export WECHAT_APP_SECRET="your_app_secret"

# 启动应用
cargo run
```

### 4. 验证配置

点击微信公众平台的"提交"按钮，系统会向你的服务器发送验证请求。如果配置正确，会显示"配置成功"。

## API端点

### GET /wechat - 服务器验证

微信会发送GET请求验证服务器：

```
GET /wechat?signature=xxx&timestamp=xxx&nonce=xxx&echostr=xxx
```

服务器验证签名后返回 `echostr` 参数。

### POST /wechat - 消息处理

微信会发送POST请求推送消息和事件：

```xml
<xml>
  <ToUserName><![CDATA[公众号]]></ToUserName>
  <FromUserName><![CDATA[用户openid]]></FromUserName>
  <CreateTime>1234567890</CreateTime>
  <MsgType><![CDATA[text]]></MsgType>
  <Content><![CDATA[用户消息内容]]></Content>
</xml>
```

## 支持的消息类型

### 文本消息

用户发送文本消息时的自动回复：

- `你好` / `hello` / `hi` → "你好！欢迎使用我们的服务！"
- `帮助` / `help` → 显示帮助信息
- `登录` → 返回登录链接
- 其他 → 通用回复

### 事件消息

- **关注事件** (`subscribe`): 用户关注时发送欢迎消息
- **取消关注事件** (`unsubscribe`): 记录取消关注（无需回复）
- **菜单点击事件** (`CLICK`): 处理自定义菜单点击

### 其他消息类型

- **图片消息**: "收到您的图片，感谢分享！"
- **语音消息**: "收到您的语音消息！"
- **其他类型**: 通用感谢回复

## 自定义消息处理

你可以在 `src/wechat_server.rs` 中自定义消息处理逻辑：

```rust
// 自定义文本消息处理
pub fn handle_text_message(&self, msg: &WeChatMessage) -> String {
    let content = msg.content.as_deref().unwrap_or("");
    
    let reply_content = match content {
        "自定义关键词" => "自定义回复内容",
        // 添加更多自定义逻辑
        _ => "默认回复",
    };

    self.create_text_response(
        &msg.from_user_name,
        &msg.to_user_name,
        reply_content,
    )
}
```

## 安全注意事项

1. **Token安全**: 确保Token足够复杂，不要泄露
2. **签名验证**: 所有请求都会验证微信签名
3. **HTTPS**: 生产环境必须使用HTTPS
4. **日志记录**: 建议记录所有消息处理日志

## 测试

### 本地测试

1. 使用ngrok暴露本地服务：
```bash
ngrok http 3000
```

2. 在微信公众平台配置ngrok提供的HTTPS地址

3. 关注你的测试公众号，发送消息测试

### 开发模式

开发模式下微信服务器功能同样可用：

```bash
export DEV_MODE=true
export WECHAT_TOKEN="dev_token"
cargo run
```

## 常见问题

**Q: 配置时提示"Token验证失败"**
A: 检查Token是否与环境变量一致，确保服务器可以正常访问

**Q: 消息无法收到**
A: 确认服务器URL配置正确，检查防火墙和网络设置

**Q: 如何处理加密消息？**
A: 设置EncodingAESKey并在代码中添加解密逻辑

**Q: 可以同时支持多个公众号吗？**
A: 当前版本支持单个公众号，多公众号需要扩展配置结构

## 扩展功能

基于当前框架，你可以轻松添加：

- 用户管理和数据存储
- 多媒体消息处理
- 模板消息推送
- 客服消息接口
- 微信支付集成
- 小程序相关功能