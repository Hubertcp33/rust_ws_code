# WebSocket Chat

## 项目概述
本项目实现了一个简单的 WebSocket 聊天应用，用户可以通过浏览器连接到 WebSocket 服务器并与其他用户进行实时聊天。项目包含前端 HTML、JavaScript 和后端 Rust 代码，使用 Warp 框架作为 WebSocket 服务器。

## 文件结构
- `static/index.html`: 前端页面文件，包含聊天界面的 HTML 结构和样式。
- `static/main.js`: 前端 JavaScript 文件，用于处理 WebSocket 连接和消息发送。
- `src/main.rs`: 后端 Rust 文件，使用 Warp 框架创建 WebSocket 服务器并处理客户端连接。

## 功能描述

### 前端功能
- **WebSocket 连接**: 使用 WebSocket 协议与服务器建立连接。
- **消息显示**: 接收到的消息会显示在页面的文本域中，并自动滚动到底部。
- **消息发送**: 用户可以通过输入框输入消息并点击发送按钮或按下回车键发送消息。
- **样式设计**: 简单的 CSS 样式，确保页面布局合理且美观。

### 后端功能
- **WebSocket 服务器**: 使用 Warp 框架创建 WebSocket 服务器，监听 `ws://localhost:7070/ws` 路径。
- **用户管理**: 使用原子计数器为每个连接的用户分配唯一 ID，并将用户信息存储在哈希表中。
- **消息广播**: 当接收到消息时，服务器会将消息广播给所有已连接的用户。
- **断开连接**: 当用户断开连接时，服务器会从用户列表中移除该用户。

## 技术栈
- **前端**:
  - HTML5
  - CSS3
  - JavaScript (ES6+)
- **后端**:
  - Rust
  - Warp 框架
  - Tokio 异步运行时
  - PrettyEnvLogger 日志库

## 运行环境
- **Rust**: 需要安装 Rust 编译工具链，可以通过 [rustup](https://rustup.rs/) 安装。
- **Node.js**: 如果需要运行前端开发服务器（可选）。
- **浏览器**: 支持 WebSocket 的现代浏览器，如 Chrome、Firefox 等。

## 启动步骤
1. **编译并运行后端服务**:
   ```bash
   cargo run
2. **在网页上打开**:
   localhost:7070
