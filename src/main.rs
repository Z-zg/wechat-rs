mod config;
mod wechat_auth;
mod wechat_server;

use askama::Template;
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Response},
    routing::{get, post},
    body::Body,
};
use config::AppConfig;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use wechat_auth::{WeChatAuth, WeChatConfig};
use wechat_server::{WeChatServerConfig, WeChatVerifyQuery, WeChatMessageQuery, WeChatMessage};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    auth_url: String,
}

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {
    user: wechat_auth::WeChatUserInfo,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    error: String,
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

struct AppState {
    wechat_auth: WeChatAuth,
    wechat_server: WeChatServerConfig,
    sessions: tokio::sync::RwLock<HashMap<String, String>>,
    dev_mode: bool,
}

async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let session_state = WeChatAuth::generate_state();
    let auth_url = if state.dev_mode {
        // 开发模式：使用本地mock回调
        format!("/dev-callback?code=mock_code&state={}", session_state)
    } else {
        state.wechat_auth.get_auth_url(&session_state)
    };

    // Store state for validation (in production, use proper session storage)
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(session_state.clone(), "pending".to_string());
    }

    let template = IndexTemplate { auth_url };
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn callback(
    Query(params): Query<CallbackQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    // Handle OAuth error
    if let Some(error) = params.error {
        let template = ErrorTemplate {
            error: format!("微信授权失败: {}", error),
        };
        let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
        return Err((StatusCode::BAD_REQUEST, Html(html)));
    }

    let code = params.code.ok_or_else(|| {
        let template = ErrorTemplate {
            error: "缺少授权码".to_string(),
        };
        let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
        (StatusCode::BAD_REQUEST, Html(html))
    })?;

    let received_state = params.state.ok_or_else(|| {
        let template = ErrorTemplate {
            error: "缺少状态参数".to_string(),
        };
        let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
        (StatusCode::BAD_REQUEST, Html(html))
    })?;

    // Validate state (in production, implement proper validation)
    {
        let sessions = state.sessions.read().await;
        if !sessions.contains_key(&received_state) {
            let template = ErrorTemplate {
                error: "无效的状态参数".to_string(),
            };
            let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
            return Err((StatusCode::BAD_REQUEST, Html(html)));
        }
    }

    // Exchange code for access token
    let auth_response = state
        .wechat_auth
        .get_access_token(&code)
        .await
        .map_err(|e| {
            let template = ErrorTemplate {
                error: format!("获取访问令牌失败: {}", e),
            };
            let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html))
        })?;

    // Get user information
    let user_info = state
        .wechat_auth
        .get_user_info(&auth_response.access_token, &auth_response.openid)
        .await
        .map_err(|e| {
            let template = ErrorTemplate {
                error: format!("获取用户信息失败: {}", e),
            };
            let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html))
        })?;

    // Clean up session
    {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&received_state);
    }

    let template = ProfileTemplate { user: user_info };
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => {
            let error_template = ErrorTemplate {
                error: "渲染用户信息失败".to_string(),
            };
            let html = error_template
                .render()
                .unwrap_or_else(|_| "渲染错误".to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, Html(html)))
        }
    }
}

// 开发模式的Mock回调处理
async fn dev_callback(
    Query(params): Query<CallbackQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    if !state.dev_mode {
        let template = ErrorTemplate {
            error: "开发模式未启用".to_string(),
        };
        let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
        return Err((StatusCode::FORBIDDEN, Html(html)));
    }

    let received_state = params.state.ok_or_else(|| {
        let template = ErrorTemplate {
            error: "缺少状态参数".to_string(),
        };
        let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
        (StatusCode::BAD_REQUEST, Html(html))
    })?;

    // Validate state
    {
        let sessions = state.sessions.read().await;
        if !sessions.contains_key(&received_state) {
            let template = ErrorTemplate {
                error: "无效的状态参数".to_string(),
            };
            let html = template.render().unwrap_or_else(|_| "渲染错误".to_string());
            return Err((StatusCode::BAD_REQUEST, Html(html)));
        }
    }

    // Mock用户信息
    let mock_user = wechat_auth::WeChatUserInfo {
        openid: "mock_openid_123456".to_string(),
        nickname: "开发测试用户".to_string(),
        sex: 1,
        province: "北京".to_string(),
        city: "北京".to_string(),
        country: "中国".to_string(),
        headimgurl: "https://via.placeholder.com/80x80?text=Mock".to_string(),
        privilege: vec![],
        unionid: Some("mock_unionid_789".to_string()),
    };

    // Clean up session
    {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&received_state);
    }

    let template = ProfileTemplate { user: mock_user };
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => {
            let error_template = ErrorTemplate {
                error: "渲染用户信息失败".to_string(),
            };
            let html = error_template
                .render()
                .unwrap_or_else(|_| "渲染错误".to_string());
            Err((StatusCode::INTERNAL_SERVER_ERROR, Html(html)))
        }
    }
}

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let app_config = match AppConfig::from_env() {
        Ok(config) => {
            if let Err(e) = config.validate() {
                eprintln!("❌ 配置验证失败: {}", e);
                std::process::exit(1);
            }
            config
        }
        Err(e) => {
            eprintln!("❌ 配置加载失败: {}", e);
            eprintln!("📝 请确保设置以下环境变量:");
            eprintln!("   WECHAT_APP_ID=你的微信应用ID");
            eprintln!("   WECHAT_APP_SECRET=你的微信应用密钥");
            eprintln!("   WECHAT_REDIRECT_URI=http://localhost:3000/callback (可选)");
            std::process::exit(1);
        }
    };

    // Initialize WeChat configuration
    let wechat_config = WeChatConfig {
        app_id: app_config.wechat_app_id.clone(),
        app_secret: app_config.wechat_app_secret.clone(),
        redirect_uri: app_config.wechat_redirect_uri.clone(),
    };

    // Initialize WeChat server configuration
    let wechat_server_config = WeChatServerConfig::new(
        app_config.wechat_token.clone(),
        app_config.wechat_app_id.clone(),
        app_config.wechat_app_secret.clone(),
    );

    let app_state = Arc::new(AppState {
        wechat_auth: WeChatAuth::new(wechat_config),
        wechat_server: wechat_server_config,
        sessions: tokio::sync::RwLock::new(HashMap::new()),
        dev_mode: app_config.dev_mode,
    });

    let app = if app_config.dev_mode {
        Router::new()
            .route("/", get(index))
            .route("/callback", get(callback))
            .route("/dev-callback", get(dev_callback))
            .route("/wechat", get(wechat_verify).post(wechat_message))
            .layer(CorsLayer::permissive())
            .with_state(app_state)
    } else {
        Router::new()
            .route("/", get(index))
            .route("/callback", get(callback))
            .route("/wechat", get(wechat_verify).post(wechat_message))
            .layer(CorsLayer::permissive())
            .with_state(app_state)
    };

    let server_addr = app_config.server_address();
    println!("🚀 服务器启动在 http://{}", server_addr);

    if app_config.dev_mode {
        println!("🔧 开发模式已启用 - 使用Mock数据");
        println!("📝 可以直接测试登录流程，无需真实微信回调");
    } else {
        println!("📱 微信应用ID: {}", app_config.wechat_app_id);
        println!("🔗 回调地址: {}", app_config.wechat_redirect_uri);
    }

    let listener = tokio::net::TcpListener::bind(&server_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// 微信服务器验证 (GET请求)
async fn wechat_verify(
    Query(params): Query<WeChatVerifyQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<String, StatusCode> {
    match state.wechat_server.handle_verify(params) {
        Ok(echostr) => Ok(echostr),
        Err(_) => Err(StatusCode::FORBIDDEN),
    }
}

// 微信消息处理 (POST请求)
async fn wechat_message(
    Query(params): Query<WeChatMessageQuery>,
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<Response<Body>, StatusCode> {
    // 验证签名
    if !state.wechat_server.verify_signature(&params.signature, &params.timestamp, &params.nonce) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 解析XML消息
    let msg: WeChatMessage = match quick_xml::de::from_str(&body) {
        Ok(msg) => msg,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // 处理消息并生成回复
    let response_xml = state.wechat_server.handle_message(msg);
    
    Ok(Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(Body::from(response_xml))
        .unwrap())
}