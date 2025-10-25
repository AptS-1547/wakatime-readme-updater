# WakaTime README Updater

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/docker/v/e1saps/wakatime-readme-updater?label=docker)](https://hub.docker.com/r/e1saps/wakatime-readme-updater)
[![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-ready-brightgreen)](https://github.com/features/actions)

> Automatically update your GitHub README with WakaTime coding statistics - Rust-powered, Docker-ready, GitHub Actions integrated.

[English](README.md) | [中文文档](README_CN.md)

## 🙏 Acknowledgments

This project is inspired by [waka-readme-stats](https://github.com/anmol098/waka-readme-stats), reimplemented in Rust for better performance, smaller binary size, and enhanced Docker support.

Special thanks to the Rust community and all open-source contributors.

## 📖 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Installation](#-installation)
- [Usage](#-usage)
  - [GitHub Actions](#github-actions-recommended)
  - [Docker](#docker)
  - [Command Line](#command-line)
- [Configuration](#️-configuration)
- [Examples](#-examples)
- [Development](#-development)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

### 🚀 Performance & Architecture
- **Rust-Powered** - Memory-safe, zero-cost abstractions, blazing fast
- **Async I/O** - Built on Tokio for efficient network operations
- **Static Linking** - Single binary, no runtime dependencies
- **Lightweight** - Optimized Docker image based on Alpine Linux

### 📊 WakaTime Integration
- Automatic fetching of WakaTime coding statistics
- Customizable time range (default: last 7 days)
- Formatted output for languages, editors, operating systems, etc.

### 🔧 Flexible Configuration
Multiple configuration methods with priority order:
1. **Command-line arguments** - For one-time runs
2. **Environment variables** - For containers and CI/CD
3. **TOML config file** - For local development

### 🐳 Docker Support
- Pre-built images on Docker Hub and GitHub Container Registry
- Multi-stage builds with cargo-chef for optimized caching
- SSL certificate support out of the box

### ⚡ GitHub Actions Ready
- Drop-in GitHub Action support
- Two usage modes:
  - With `actions/checkout` (recommended)
  - Auto-clone repository (checkout-less)
- Automatic commit and push
- Smart Git authentication and permission handling

## 🚀 Quick Start

### Using GitHub Actions (Recommended)

1. Add markers to your `README.md`:

```markdown
<!--START_SECTION:waka-->
<!--END_SECTION:waka-->
```

2. Create `.github/workflows/wakatime.yml`:

```yaml
name: Update WakaTime Stats

on:
  schedule:
    - cron: '0 0 * * *'  # Daily updates
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

3. Add your WakaTime API key to repository secrets as `WAKATIME_API_KEY`

That's it! Your README will be automatically updated daily.

## 📦 Installation

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

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/AptS-1547/wakatime-readme-updater/releases)

Supported platforms:
- Linux (x86_64, aarch64)
- macOS (x86_64, Apple Silicon)
- Windows (x86_64)

### Build from Source

```bash
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater
cargo build --release
```

Binary will be at `target/release/wakatime-readme-updater`

## 🔧 Usage

### GitHub Actions (Recommended)

#### Basic Usage

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
  with:
    wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
    gh_token: ${{ secrets.GITHUB_TOKEN }}
```

#### Advanced Configuration

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

#### Without Checkout Action

```yaml
- uses: AptS-1547/wakatime-readme-updater@v1
  with:
    wakatime_api_key: ${{ secrets.WAKATIME_API_KEY }}
    gh_token: ${{ secrets.GITHUB_TOKEN }}
    repository: ${{ github.repository }}  # Auto-clone
```

### Docker

```bash
docker run --rm \
  -v $(pwd):/repo \
  -e INPUT_WAKATIME_API_KEY=your_wakatime_api_key \
  -e INPUT_GH_TOKEN=your_github_token \
  ghcr.io/apts-1547/wakatime-readme-updater:latest
```

With custom configuration:

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

### Command Line

```bash
wakatime-readme-updater \
  --api-key YOUR_WAKATIME_API_KEY \
  --readme-path README.md \
  --section-name waka \
  --stats-range 7
```

### TOML Configuration

Create `wakatime-updater.toml`:

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

Then run:

```bash
wakatime-readme-updater --config-path wakatime-updater.toml
```

## ⚙️ Configuration

### All Available Parameters

| Parameter | Description | Default | Required |
|-----------|-------------|---------|----------|
| `wakatime_api_key` | WakaTime API Key | - | ✅ |
| `gh_token` | GitHub Token for pushing | - | ✅ |
| `readme_path` | Path to README file | `README.md` | ❌ |
| `section_name` | Section marker name | `waka` | ❌ |
| `stats_range` | Number of days for stats | `7` | ❌ |
| `auto_commit` | Auto-commit changes | `true` | ❌ |
| `auto_push` | Auto-push to remote | `true` | ❌ |
| `commit_username` | Git commit username | `github-actions[bot]` | ❌ |
| `commit_email` | Git commit email | `github-actions[bot]@users.noreply.github.com` | ❌ |
| `commit_message` | Commit message | `📊 Updated WakaTime stats` | ❌ |
| `repository` | Repository to clone (for checkout-less mode) | Auto-detected | ❌ |

### Configuration Priority

```
Command-line args > Environment variables > TOML config > Defaults
```

### Getting WakaTime API Key

1. Go to [WakaTime Settings](https://wakatime.com/settings/account)
2. Scroll down to "API Key" section
3. Copy your API key
4. Add it to GitHub repository secrets as `WAKATIME_API_KEY`

## 📋 Examples

### Example Output

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

### Real-World Examples

Check out these profiles using WakaTime README Updater:
- [AptS-1547's Profile](https://github.com/AptS-1547)

## 🛠️ Development

### Prerequisites

- Rust 1.90+
- Cargo
- Docker (optional)

### Development Setup

```bash
# Clone repository
git clone https://github.com/AptS-1547/wakatime-readme-updater.git
cd wakatime-readme-updater

# Run in development mode
cargo run

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Build Optimization

The project uses different optimization profiles:

- **Development**: Fast compilation, debug symbols
- **Release**: Size optimization (`opt-level = "s"`), LTO enabled
- **Debug-Release**: Optimized with debug symbols

### Docker Development

```bash
# Build Docker image
docker build -t wakatime-readme-updater .

# Run locally
docker run --rm -v $(pwd):/repo wakatime-readme-updater
```

## 🏗️ Architecture

```
wakatime-readme-updater/
├── src/
│   ├── cli/           # Command-line interface
│   ├── config/        # Configuration management
│   ├── formatter/     # Stats formatting
│   ├── git/           # Git operations
│   │   ├── cloner.rs  # Repository cloning
│   │   ├── committer.rs # Commit operations
│   │   └── pusher.rs  # Push operations
│   ├── readme/        # README manipulation
│   └── wakatime/      # WakaTime API client
├── Dockerfile         # Docker build
├── action.yml         # GitHub Action metadata
└── Cargo.toml         # Rust dependencies
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Contribution Guidelines

- Follow Rust naming conventions and best practices
- Add tests for new features
- Update documentation as needed
- Ensure `cargo fmt` and `cargo clippy` pass

### Contributors

Thanks goes to these wonderful people:

<a href="https://github.com/AptS-1547/wakatime-readme-updater/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=AptS-1547/wakatime-readme-updater" alt="Contributors" />
</a>

Made with [contrib.rocks](https://contrib.rocks).

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📮 Contact

- Author: AptS-1547
- Email: apts-1547@esaps.net
- GitHub: [@AptS-1547](https://github.com/AptS-1547)

## 🌟 Star History

If you find this project helpful, please consider giving it a star ⭐

## 📊 Project Status

This project is actively maintained and welcomes contributions!

---

**Built with ❤️ and Rust 🦀 by AptS:1547**

[Docker Hub](https://hub.docker.com/r/e1saps/wakatime-readme-updater) | [GitHub Container Registry](https://github.com/AptS-1547/wakatime-readme-updater/pkgs/container/wakatime-readme-updater) | [Release Notes](https://github.com/AptS-1547/wakatime-readme-updater/releases)
