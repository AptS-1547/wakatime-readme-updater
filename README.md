# WakaTime README Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/docker/v/e1saps/wakatime-readme-updater?label=docker)](https://hub.docker.com/r/e1saps/wakatime-readme-updater)
[![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-ready-brightgreen)](https://github.com/features/actions)

> Automatically update your GitHub README with WakaTime coding statistics - Rust-powered, Docker-ready, GitHub Actions integrated.

[English](README.md) | [中文文档](README_CN.md)

Inspired by [waka-readme-stats](https://github.com/anmol098/waka-readme-stats), reimplemented in Rust for better performance and smaller binary size.

## ✨ Features

- **Rust-Powered** - Memory-safe, blazing fast, single binary with no runtime dependencies
- **Multiple Deployment Options** - GitHub Actions, Docker, or command-line
- **Flexible Configuration** - Command-line args, environment variables, or TOML config
- **Auto Git Operations** - Automatic commit and push with smart authentication
- **Customizable Stats** - Languages, editors, operating systems, and more

## 🚀 Quick Start

1. Add markers to your `README.md`:
   ```markdown
   <!--START_SECTION:waka-->
   <!--END_SECTION:waka-->
   ```

2. Create `.github/workflows/update-wakatime-stats.yml`:
   ```yaml
   name: Update WakaTime Stats

   on:
     schedule:
       - cron: '0 0 * * *'  # Run every day at 00:00 UTC
     workflow_dispatch:  # Allow manual trigger

   permissions:
     contents: write  # Required for committing and pushing changes

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
             # Optional: customize settings below
             # readme_path: 'README.md'
             # section_name: 'waka'
             # stats_range: '7'
             # commit_message: '📊 Updated WakaTime stats'
   ```

3. Add your [WakaTime API key](https://wakatime.com/settings/account) to repository secrets as `WAKATIME_API_KEY`

Done! Your README will be automatically updated daily.

## 📦 Other Installation Methods

### Docker

```bash
docker pull ghcr.io/apts-1547/wakatime-readme-updater:latest

docker run --rm \
  -v $(pwd):/repo \
  -e INPUT_WAKATIME_API_KEY=your_key \
  -e INPUT_GH_TOKEN=your_token \
  ghcr.io/apts-1547/wakatime-readme-updater:latest
```

### Command Line

Download from [GitHub Releases](https://github.com/AptS-1547/wakatime-readme-updater/releases) or build from source:

```bash
cargo install --git https://github.com/AptS-1547/wakatime-readme-updater

wakatime-readme-updater \
  --api-key YOUR_WAKATIME_API_KEY \
  --readme-path README.md
```

## ⚙️ Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `wakatime_api_key` | WakaTime API Key (required) | - |
| `gh_token` | GitHub Token (required) | - |
| `readme_path` | Path to README file | `README.md` |
| `section_name` | Section marker name | `waka` |
| `stats_range` | Number of days for stats | `7` |
| `commit_message` | Commit message | `📊 Updated WakaTime stats` |
| `commit_username` | Git commit username | `github-actions[bot]` |
| `commit_email` | Git commit email | `github-actions[bot]@users.noreply.github.com` |
| `repository` | Repository to clone (checkout-less mode) | Auto-detected |

## 📋 Example Output

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

See it in action: [AptS-1547's Profile](https://github.com/AptS-1547)

## 🛠️ Development

```bash
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater
cargo build --release
```

## 🤝 Contributing

Contributions welcome! Please ensure `cargo fmt` and `cargo clippy` pass before submitting PRs.

### Contributors

<a href="https://github.com/AptS-1547/wakatime-readme-updater/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=AptS-1547/wakatime-readme-updater" alt="Contributors" />
</a>

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

**Built with Rust 🦀 by [AptS-1547](https://github.com/AptS-1547)**
