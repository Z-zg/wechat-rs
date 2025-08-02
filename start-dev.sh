#!/bin/bash

# WeChat Auth App 开发模式启动脚本

echo "🔧 启动微信认证应用 - 开发模式"

# 设置开发模式环境变量
export DEV_MODE=true
export WECHAT_APP_ID="dev_app_id"
export WECHAT_APP_SECRET="dev_app_secret"
export WECHAT_REDIRECT_URI="http://localhost:3000/dev-callback"
export WECHAT_TOKEN="dev_token"

echo "✅ 开发模式配置:"
echo "   DEV_MODE=true"
echo "   使用Mock数据，无需真实微信回调"

# 构建项目
echo "🔨 构建项目..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ 构建失败"
    exit 1
fi

echo "✅ 构建成功"

# 启动应用
echo "🌐 启动Web服务器..."
echo "📝 访问 http://localhost:3000 开始测试"
echo "🔧 开发模式：点击登录按钮将使用Mock数据"
cargo run --release