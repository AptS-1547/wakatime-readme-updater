# WakaTime README Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/docker/v/e1saps/wakatime-readme-updater?label=docker)](https://hub.docker.com/r/e1saps/wakatime-readme-updater)
[![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-ready-brightgreen)](https://github.com/features/actions)

> 自动更新 GitHub README 中的 WakaTime 编程统计 - Rust 驱动，Docker 就绪，GitHub Actions 集成

[English](README.md) | [中文文档](README_CN.md)

受 [waka-readme-stats](https://github.com/anmol098/waka-readme-stats) 启发，使用 Rust 重新实现以获得更好的性能和更小的二进制体积。

## ✨ 特性

- **Rust 驱动** - 内存安全，极速性能，单一二进制文件无运行时依赖
- **多种部署方式** - GitHub Actions、Docker 或命令行
- **灵活配置** - 命令行参数、环境变量或 TOML 配置文件
- **自动 Git 操作** - 自动提交和推送，智能认证处理
- **可自定义统计** - 语言、编辑器、操作系统等

## 🚀 快速开始

1. 在你的 `README.md` 中添加标记：
   ```markdown
   <!--START_SECTION:waka-->
   <!--END_SECTION:waka-->
   ```

2. 创建 `.github/workflows/update-wakatime-stats.yml`：
   ```yaml
   name: Update WakaTime Stats

   on:
     schedule:
       - cron: '0 0 * * *'  # 每天 00:00 UTC 运行
     workflow_dispatch:  # 允许手动触发

   permissions:
     contents: write  # 提交和推送所需权限

   jobs:
     update-readme:
       runs-on: ubuntu-latest
       steps:
         - name: Update WakaTime Stats
           uses: AptS-1547/wakatime-readme-updater@v1
           with:
             wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
             gh_token: ${{ secrets.GITHUB_TOKEN }}
             repository: ${{ github.repository }}
             # 可选：自定义以下设置
             # readme_path: 'README.md'
             # section_name: 'waka'
             # stats_range: '7'
             # commit_message: '📊 Updated WakaTime stats'
   ```

3. 将你的 [WakaTime API 密钥](https://wakatime.com/settings/account) 添加到仓库 secrets，命名为 `WAKATIME_API_KEY`

完成！你的 README 将每天自动更新。

## 📦 其他安装方式

### Docker

```bash
docker pull ghcr.io/apts-1547/wakatime-readme-updater:latest

docker run --rm \
  -v $(pwd):/repo \
  -e INPUT_WAKATIME_API_KEY=your_key \
  -e INPUT_GH_TOKEN=your_token \
  ghcr.io/apts-1547/wakatime-readme-updater:latest
```

### 命令行

从 [GitHub Releases](https://github.com/AptS-1547/wakatime-readme-updater/releases) 下载或从源码构建：

```bash
cargo install --git https://github.com/AptS-1547/wakatime-readme-updater

wakatime-readme-updater \
  --api-key YOUR_WAKATIME_API_KEY \
  --readme-path README.md
```

## ⚙️ 配置

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `wakatime_api_key` | WakaTime API 密钥（必填） | - |
| `gh_token` | GitHub Token（必填） | - |
| `readme_path` | README 文件路径 | `README.md` |
| `section_name` | 区域标记名称 | `waka` |
| `stats_range` | 统计天数 | `7` |
| `commit_message` | 提交信息 | `📊 Updated WakaTime stats` |
| `commit_username` | Git 提交用户名 | `github-actions[bot]` |
| `commit_email` | Git 提交邮箱 | `github-actions[bot]@users.noreply.github.com` |
| `repository` | 要克隆的仓库（checkout-less 模式） | 自动检测 |

## 📋 输出示例

```markdown
<!--START_SECTION:waka-->
📊 **This Week I Spent My Time On:**

\`\`\`text
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
\`\`\`
<!--END_SECTION:waka-->
```

查看实际效果：[AptS-1547's Profile](https://github.com/AptS-1547)

## 🛠️ 开发

```bash
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater
cargo build --release
```

## 🤝 贡献

欢迎贡献！提交 PR 前请确保 `cargo fmt` 和 `cargo clippy` 通过。

### 贡献者

<a href="https://github.com/AptS-1547/wakatime-readme-updater/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=AptS-1547/wakatime-readme-updater" alt="贡献者" />
</a>

## 📄 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

**由 [AptS-1547](https://github.com/AptS-1547) 用 Rust 🦀 构建**
