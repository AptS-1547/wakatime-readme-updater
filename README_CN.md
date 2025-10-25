# WakaTime README Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/docker/v/e1saps/wakatime-readme-updater?label=docker)](https://hub.docker.com/r/e1saps/wakatime-readme-updater)
[![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-ready-brightgreen)](https://github.com/features/actions)

> 自动更新 GitHub README 中的 WakaTime 编程统计 - Rust 驱动，Docker 就绪，GitHub Actions 集成

[English](README.md) | [中文文档](README_CN.md)

## 🙏 致谢

本项目受到 [waka-readme-stats](https://github.com/anmol098/waka-readme-stats) 的启发，使用 Rust 重新实现以获得更好的性能、更小的二进制体积和增强的 Docker 支持。

特别感谢 Rust 社区和所有开源贡献者。

## 📖 目录

- [特性](#-特性)
- [快速开始](#-快速开始)
- [安装](#-安装)
- [使用方法](#-使用方法)
  - [GitHub Actions](#github-actions推荐)
  - [Docker](#docker)
  - [命令行](#命令行)
- [配置](#-配置)
- [示例](#-示例)
- [开发](#-开发)
- [贡献](#-贡献)
- [许可证](#-许可证)

## ✨ 特性

### 🚀 性能与架构

- **Rust 驱动** - 内存安全，零成本抽象，极速性能
- **异步 I/O** - 基于 Tokio 构建，高效的网络操作
- **静态链接** - 单一二进制文件，无运行时依赖
- **轻量级** - 基于 Alpine Linux 的优化 Docker 镜像

### 📊 WakaTime 集成

- 自动获取 WakaTime 编程统计数据
- 可自定义时间范围（默认：最近 7 天）
- 格式化输出语言、编辑器、操作系统等统计信息

### 🔧 灵活配置

支持多种配置方式，优先级顺序：

1. **命令行参数** - 适合一次性运行
2. **环境变量** - 适合容器和 CI/CD
3. **TOML 配置文件** - 适合本地开发

### 🐳 Docker 支持

- Docker Hub 和 GitHub Container Registry 预构建镜像
- 使用 cargo-chef 的多阶段构建优化缓存
- 开箱即用的 SSL 证书支持

### ⚡ GitHub Actions 就绪

- 即插即用的 GitHub Action 支持
- 两种使用模式：
  - 配合 `actions/checkout` 使用（推荐）
  - 自动克隆仓库（无需 checkout）
- 自动提交和推送
- 智能 Git 认证和权限处理

## 🚀 快速开始

### 使用 GitHub Actions（推荐）

1. 在你的 `README.md` 中添加标记：

```markdown
<!--START_SECTION:waka-->
<!--END_SECTION:waka-->
```

2. 创建 `.github/workflows/wakatime.yml`：

```yaml
name: Update WakaTime Stats

on:
  schedule:
    - cron: '0 0 * * *'  # 每天更新
  workflow_dispatch:

permissions:
  contents: write

jobs:
  update-stats:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Update WakaTime Stats
        uses: AptS-1547/wakatime-readme-updater@v1
        with:
          wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
          gh_token: ${{ secrets.GITHUB_TOKEN }}
```

3. 将你的 WakaTime API 密钥添加到仓库 secrets，命名为 `WAKATIME_API_KEY`

完成！你的 README 将每天自动更新。

## 📦 安装

### GitHub Actions

```yaml
uses: AptS-1547/wakatime-readme-updater@v1
```

### Docker

```bash
# Docker Hub
docker pull e1saps/wakatime-readme-updater:latest

# GitHub Container Registry
docker pull ghcr.io/apts-1547/wakatime-readme-updater:latest
```

### 预构建二进制文件

从 [GitHub Releases](https://github.com/AptS-1547/wakatime-readme-updater/releases) 下载

支持的平台：

- Linux (x86_64, aarch64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

### 从源码构建

```bash
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater
cargo build --release
```

二进制文件位于 `target/release/wakatime-readme-updater`

## 🔧 使用方法

### GitHub Actions（推荐）

#### 基础使用

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
  with:
    wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
    gh_token: ${{ secrets.GITHUB_TOKEN }}
```

#### 高级配置

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
  with:
    wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
    gh_token: ${{ secrets.GITHUB_TOKEN }}
    readme_path: 'README.md'
    section_name: 'waka'
    stats_range: '7'
    commit_message: '📊 Updated WakaTime stats'
    commit_username: 'github-actions[bot]'
    commit_email: 'github-actions[bot]@users.noreply.github.com'
```

#### 不使用 Checkout Action

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
  with:
    wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
    gh_token: ${{ secrets.GITHUB_TOKEN }}
    repository: ${{ github.repository }}  # 自动克隆
```

### Docker

```bash
docker run --rm \
  -v $(pwd):/repo \
  -e INPUT_WAKATIME_API_KEY=your_wakatime_api_key \
  -e INPUT_GH_TOKEN=your_github_token \
  ghcr.io/apts-1547/wakatime-readme-updater:latest
```

使用自定义配置：

```bash
docker run --rm \
  -v $(pwd):/repo \
  -e INPUT_WAKATIME_API_KEY=your_key \
  -e INPUT_GH_TOKEN=your_token \
  -e INPUT_README_PATH=README.md \
  -e INPUT_SECTION_NAME=waka \
  -e INPUT_STATS_RANGE=7 \
  -e INPUT_AUTO_COMMIT=true \
  -e INPUT_AUTO_PUSH=true \
  ghcr.io/apts-1547/wakatime-readme-updater:latest
```

### 命令行

```bash
wakatime-readme-updater \
  --api-key YOUR_WAKATIME_API_KEY \
  --readme-path README.md \
  --section-name waka \
  --stats-range 7
```

### TOML 配置文件

创建 `wakatime-updater.toml`：

```toml
api_key = "your_wakatime_api_key"
readme_path = "README.md"
section_name = "waka"
stats_range = 7
auto_commit = true
auto_push = true
git_user_name = "Your Name"
git_user_email = "your.email@example.com"
github_token = "your_github_token"
commit_message = "📊 Updated WakaTime stats"
```

然后运行：

```bash
wakatime-readme-updater --config-path wakatime-updater.toml
```

## ⚙️ 配置

### 所有可用参数

| 参数 | 说明 | 默认值 | 必填 |
|------|------|--------|------|
| `wakatime_api_key` | WakaTime API 密钥 | - | ✅ |
| `gh_token` | GitHub Token（用于推送） | - | ✅ |
| `readme_path` | README 文件路径 | `README.md` | ❌ |
| `section_name` | 区域标记名称 | `waka` | ❌ |
| `stats_range` | 统计天数 | `7` | ❌ |
| `auto_commit` | 自动提交更改 | `true` | ❌ |
| `auto_push` | 自动推送到远程 | `true` | ❌ |
| `commit_username` | Git 提交用户名 | `github-actions[bot]` | ❌ |
| `commit_email` | Git 提交邮箱 | `github-actions[bot]@users.noreply.github.com` | ❌ |
| `commit_message` | 提交信息 | `📊 Updated WakaTime stats` | ❌ |
| `repository` | 要克隆的仓库（checkout-less 模式） | 自动检测 | ❌ |

### 配置优先级

```text
命令行参数 > 环境变量 > TOML 配置 > 默认值
```

### 获取 WakaTime API 密钥

1. 访问 [WakaTime Settings](https://wakatime.com/settings/account)
2. 向下滚动到 "API Key" 部分
3. 复制你的 API 密钥
4. 将其添加到 GitHub 仓库 secrets，命名为 `WAKATIME_API_KEY`

## 📋 示例

### 输出示例

```markdown
<!--START_SECTION:waka-->
📊 **This Week I Spent My Time On:**

```text
💬 Programming Languages:
Rust         12 hrs 34 mins  ████████████░░░░░░░░  60.5%
TypeScript   4 hrs 12 mins   ████░░░░░░░░░░░░░░░░  20.2%
Go           2 hrs 45 mins   ██░░░░░░░░░░░░░░░░░░  13.3%
Python       1 hr 15 mins    █░░░░░░░░░░░░░░░░░░░   6.0%

💻 Editors:
VS Code      18 hrs 23 mins  ██████████████████░░  88.5%
Vim          2 hrs 23 mins   ███░░░░░░░░░░░░░░░░░  11.5%

💻 Operating Systems:
Linux        15 hrs 12 mins  ███████████████░░░░░  73.2%
macOS        5 hrs 34 mins   ██████░░░░░░░░░░░░░░  26.8%
```

<!--END_SECTION:waka-->
```

### 真实案例

查看这些使用 WakaTime README Updater 的 Profile：

- [AptS-1547's Profile](https://github.com/AptS-1547)

## 🛠️ 开发

### 前置要求

- Rust 1.90+
- Cargo
- Docker（可选）

### 开发设置

```bash
# 克隆仓库
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater

# 运行开发模式
cargo run

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 代码检查
cargo clippy
```

### 构建优化

项目使用不同的优化配置：

- **开发模式**：快速编译，保留调试符号
- **发布模式**：体积优化（`opt-level = "s"`），启用 LTO
- **调试发布模式**：优化的同时保留调试符号

### Docker 开发

```bash
# 构建 Docker 镜像
docker build -t wakatime-readme-updater .

# 本地运行
docker run --rm -v $(pwd):/repo wakatime-readme-updater
```

## 🏗️ 架构

```text
wakatime-readme-updater/
├── src/
│   ├── cli/           # 命令行界面
│   ├── config/        # 配置管理
│   ├── formatter/     # 统计格式化
│   ├── git/           # Git 操作
│   │   ├── cloner.rs  # 仓库克隆
│   │   ├── committer.rs # 提交操作
│   │   └── pusher.rs  # 推送操作
│   ├── readme/        # README 操作
│   └── wakatime/      # WakaTime API 客户端
├── Dockerfile         # Docker 构建
├── action.yml         # GitHub Action 元数据
└── Cargo.toml         # Rust 依赖
```

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建你的特性分支（`git checkout -b feature/amazing-feature`）
3. 提交你的更改（`git commit -m 'Add some amazing feature'`）
4. 推送到分支（`git push origin feature/amazing-feature`）
5. 开启一个 Pull Request

### 贡献指南

- 遵循 Rust 命名约定和最佳实践
- 为新功能添加测试
- 根据需要更新文档
- 确保 `cargo fmt` 和 `cargo clippy` 通过

### 贡献者

感谢所有贡献者：

<a href="https://github.com/AptS-1547/wakatime-readme-updater/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=AptS-1547/wakatime-readme-updater" alt="贡献者" />
</a>

使用 [contrib.rocks](https://contrib.rocks) 生成。

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 📮 联系方式

- 作者：AptS-1547
- 邮箱：<apts-1547@esaps.net>
- GitHub：[@AptS-1547](https://github.com/AptS-1547)

## 🌟 Star 历史

如果你觉得这个项目有帮助，请考虑给它一个 star ⭐

## 📊 项目状态

本项目正在积极维护中，欢迎贡献！

---

**由 AptS:1547 用 ❤️ 和 Rust 🦀 构建**

[Docker Hub](https://hub.docker.com/r/e1saps/wakatime-readme-updater) | [GitHub Container Registry](https://github.com/AptS-1547/wakatime-readme-updater/pkgs/container/wakatime-readme-updater) | [发布说明](https://github.com/AptS-1547/wakatime-readme-updater/releases)
