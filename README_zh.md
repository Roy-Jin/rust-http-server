# HTTP-SERVER

一个用Rust语言编写的轻量级HTTP服务器。该项目旨在提供一个简单、高效的文件服务器，适用于本地开发和测试。

[English](README.md) | [中文](README_zh.md)

## 特性

- 🚀 **快速且异步**: 基于 [Axum](https://github.com/tokio-rs/axum) 和 [Tokio](https://tokio.rs/) 构建，性能出色
- 📁 **目录列表**: 美观、现代化的目录浏览界面
- 🔒 **路径安全处理**: 安全的路径处理，防止目录遍历攻击
- 🌐 **CORS 支持**: 默认启用跨域资源共享
- 📦 **GZIP 压缩**: 自动压缩以减小传输负载
- 📄 **自定义 404 页面**: 支持自定义 404.html 文件
- 🎨 **多彩命令行**: 美观的终端输出，带有彩色状态消息
- 🌍 **自动打开浏览器**: 服务器启动时自动打开浏览器

## 安装

### 从源码安装

```bash
git clone https://github.com/Roy-Jin/rust-http-server.git
cd rust-http-server
cargo build --release
```

编译后的二进制文件位于 `target/release/http-server`。

## 使用方法

### 基本使用

在 80 端口上启动服务器，服务当前目录：

```bash
.\http-server
```

### 自定义端口和目录

```bash
.\http-server -p 8080 -r ./public
```

### 命令行选项

```
用法:
    http-server [选项]

选项:
    -r, --root <ROOT>           要服务的根目录 [默认: .]
    -a, --address <ADDRESS>     绑定的地址 [默认: 0.0.0.0]
    -p, --port <PORT>           监听的端口号 [默认: 80]
        --no-dir-listing        禁用目录列表
        --no-cors               禁用 CORS
        --no-gzip               禁用 GZIP 压缩
        --not-found-page <PATH> 自定义 404 页面 [默认: 404.html]
    -o, --open                  自动打开浏览器
    -h, --help                  打印帮助信息
    -v, --version               打印版本信息
```

## 示例

### 服务指定目录

```bash
.\http-server -r /path/to/your/directory
```

### 禁用目录列表

```bash
.\http-server --no-dir-listing
```

### 禁用 CORS

```bash
.\http-server --no-cors
```

### 使用自定义 404 页面

```bash
.\http-server --not-found-page my-404.html
```

### 自动打开浏览器

```bash
.\http-server -o
```

## 工作原理

1. **文件服务**: 从指定的根目录提供静态文件服务
2. **索引文件**: 如果目录中存在 `index.html`，则自动提供该文件
3. **目录列表**: 当找不到 index.html 时，显示美观的目录浏览器
4. **路径处理**: 清理所有路径以防止目录遍历攻击
5. **压缩**: 在支持的情况下自动使用 GZIP 压缩响应
6. **日志记录**: 记录所有请求，包含状态码和响应时间

## 技术栈

- **[Axum](https://github.com/tokio-rs/axum)**: Web 框架
- **[Tokio](https://tokio.rs/)**: 异步运行时
- **[tower-http](https://github.com/tower-rs/tower-http)**: HTTP 中间件（CORS、压缩）
- **[clap](https://github.com/clap-rs/clap)**: 命令行参数解析
- **[colored](https://github.com/mackwic/colored)**: 终端彩色输出
- **[open](https://github.com/Byron/open-rs)**: 自动打开浏览器

## 许可证

本项目基于 [MIT License](LICENSE) 发布

## 作者

由 [Roy-Jin](https://github.com/Roy-Jin) 创建