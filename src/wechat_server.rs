use serde::Deserialize;
use sha1::{Sha1, Digest};

/// 微信服务器配置验证参数
#[derive(Debug, Deserialize)]
pub struct WeChatVerifyQuery {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: String,
}

/// 微信消息推送参数
#[derive(Debug, Deserialize)]
pub struct WeChatMessageQuery {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub openid: Option<String>,
    pub encrypt_type: Option<String>,
    pub msg_signature: Option<String>,
}

/// 微信XML消息结构
#[derive(Debug, Deserialize)]
pub struct WeChatMessage {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Content")]
    pub content: Option<String>,
    #[serde(rename = "MsgId")]
    pub msg_id: Option<u64>,
    #[serde(rename = "Event")]
    pub event: Option<String>,
    #[serde(rename = "EventKey")]
    pub event_key: Option<String>,
}

/// 微信服务器配置
pub struct WeChatServerConfig {
    pub token: String,
    pub app_id: String,
    pub app_secret: String,
    pub encoding_aes_key: Option<String>,
}

impl WeChatServerConfig {
    pub fn new(token: String, app_id: String, app_secret: String) -> Self {
        Self {
            token,
            app_id,
            app_secret,
            encoding_aes_key: None,
        }
    }

    /// 验证微信服务器签名
    pub fn verify_signature(&self, signature: &str, timestamp: &str, nonce: &str) -> bool {
        let mut params = vec![&self.token, timestamp, nonce];
        params.sort();
        
        let mut hasher = Sha1::new();
        hasher.update(params.join(""));
        let result = hasher.finalize();
        let calculated_signature = hex::encode(result);
        
        calculated_signature == signature
    }

    /// 处理微信服务器验证请求 (GET)
    pub fn handle_verify(&self, query: WeChatVerifyQuery) -> Result<String, String> {
        if self.verify_signature(&query.signature, &query.timestamp, &query.nonce) {
            Ok(query.echostr)
        } else {
            Err("签名验证失败".to_string())
        }
    }

    /// 生成文本回复消息
    pub fn create_text_response(&self, to_user: &str, from_user: &str, content: &str) -> String {
        let timestamp = chrono::Utc::now().timestamp();
        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[{}]]></Content>
</xml>"#,
            to_user, from_user, timestamp, content
        )
    }

    /// 处理文本消息
    pub fn handle_text_message(&self, msg: &WeChatMessage) -> String {
        let content = msg.content.as_deref().unwrap_or("");
        
        let reply_content = match content {
            "你好" | "hello" | "hi" => "你好！欢迎使用我们的服务！",
            "帮助" | "help" => "可用命令：\n- 你好：问候\n- 帮助：显示此帮助\n- 登录：获取登录链接",
            "登录" => "请访问我们的网站进行登录：http://your-domain.com",
            _ => "感谢您的消息！如需帮助，请回复\"帮助\"。",
        };

        self.create_text_response(
            &msg.from_user_name,
            &msg.to_user_name,
            reply_content,
        )
    }

    /// 处理关注事件
    pub fn handle_subscribe_event(&self, msg: &WeChatMessage) -> String {
        let welcome_msg = "欢迎关注我们！\n\n您可以：\n- 回复\"登录\"获取登录链接\n- 回复\"帮助\"查看更多功能";
        
        self.create_text_response(
            &msg.from_user_name,
            &msg.to_user_name,
            welcome_msg,
        )
    }

    /// 处理取消关注事件
    pub fn handle_unsubscribe_event(&self, _msg: &WeChatMessage) -> String {
        // 取消关注事件不需要回复，微信不会推送给用户
        "success".to_string()
    }

    /// 处理事件消息
    pub fn handle_event_message(&self, msg: &WeChatMessage) -> String {
        match msg.event.as_deref() {
            Some("subscribe") => self.handle_subscribe_event(msg),
            Some("unsubscribe") => self.handle_unsubscribe_event(msg),
            Some("CLICK") => {
                // 处理菜单点击事件
                let reply = match msg.event_key.as_deref() {
                    Some("LOGIN") => "请访问：http://your-domain.com 进行登录",
                    Some("HELP") => "如需帮助，请联系客服",
                    _ => "感谢您的操作！",
                };
                self.create_text_response(&msg.from_user_name, &msg.to_user_name, reply)
            }
            _ => self.create_text_response(
                &msg.from_user_name,
                &msg.to_user_name,
                "感谢您的关注！",
            ),
        }
    }

    /// 处理微信消息
    pub fn handle_message(&self, msg: WeChatMessage) -> String {
        match msg.msg_type.as_str() {
            "text" => self.handle_text_message(&msg),
            "event" => self.handle_event_message(&msg),
            "image" => self.create_text_response(
                &msg.from_user_name,
                &msg.to_user_name,
                "收到您的图片，感谢分享！",
            ),
            "voice" => self.create_text_response(
                &msg.from_user_name,
                &msg.to_user_name,
                "收到您的语音消息！",
            ),
            _ => self.create_text_response(
                &msg.from_user_name,
                &msg.to_user_name,
                "感谢您的消息！",
            ),
        }
    }
}