#!/bin/bash

# WeChat Auth App 启动脚本

echo "🚀 启动微信认证应用..."

# 检查环境变量
if [ -z "$WECHAT_APP_ID" ]; then
    echo "❌ 错误: 请设置 WECHAT_APP_ID 环境变量"
    exit 1
fi

if [ -z "$WECHAT_APP_SECRET" ]; then
    echo "❌ 错误: 请设置 WECHAT_APP_SECRET 环境变量"
    exit 1
fi

if [ -z "$WECHAT_REDIRECT_URI" ]; then
    echo "⚠️  警告: 未设置 WECHAT_REDIRECT_URI，使用默认值"
    export WECHAT_REDIRECT_URI="http://localhost:3000/callback"
fi

echo "✅ 环境变量检查完成"
echo "📱 App ID: $WECHAT_APP_ID"
echo "🔗 Redirect URI: $WECHAT_REDIRECT_URI"

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
cargo run --release