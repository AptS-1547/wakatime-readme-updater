# WakaTime README Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

一个用 Rust 编写的轻量级工具，用于自动更新 GitHub README 中的 WakaTime 编码统计信息。

## 🚀 特性

- 🦀 使用 Rust 编写，性能优异
- 📊 自动同步 WakaTime 编码统计
- 🎨 支持自定义统计展示格式
- 🔄 可集成到 GitHub Actions 工作流
- ⚡ 编译后体积小，执行速度快

## 📦 安装

### 从源码编译

```bash
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater
cargo build --release
```

编译后的二进制文件位于 `target/release/wakatime-readme-updater`

### 使用 Docker

```bash
docker build -t wakatime-readme-updater .
docker run wakatime-readme-updater
```

## 🔧 使用方法

> ⚠️ 本项目目前处于早期开发阶段 (v0.0.1-alpha.1)

### 基本使用

```bash
wakatime-readme-updater --api-key YOUR_WAKATIME_API_KEY
```

### GitHub Actions 集成

在你的仓库中创建 `.github/workflows/wakatime.yml`：

```yaml
name: Update WakaTime Stats

on:
  schedule:
    - cron: '0 0 * * *'  # 每天更新
  workflow_dispatch:

jobs:
  update-readme:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Update WakaTime Stats
        uses: apts-1547/wakatime-readme-updater@master
        with:
          WAKATIME_API_KEY: ${{ secrets.WAKATIME_API_KEY }}
```

## 📝 配置

在你的 README.md 中添加占位符：

```markdown
<!--START_SECTION:waka-->
<!--END_SECTION:waka-->
```

工具将自动更新这两个注释之间的内容。

## 🛠️ 开发

### 开发环境要求

- Rust 1.70+
- Cargo

### 运行开发版本

```bash
cargo run
```

### 优化配置

项目包含针对不同场景的优化配置：

- **开发模式**: 快速编译，保留调试信息
- **发布模式**: 体积优化 (`opt-level = "s"`)，启用 LTO
- **调试发布模式**: 保留调试符号的优化版本

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

本项目受到 [waka-readme-stats](https://github.com/anmol098/waka-readme-stats) 的启发，使用 Rust 重新实现以获得更好的性能和更小的体积。

## 📮 联系方式

- 作者: AptS-1547
- Email: apts-1547@esaps.net

---

⚡ Made with Rust 🦀
