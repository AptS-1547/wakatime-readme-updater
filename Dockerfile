# 多阶段构建 - cargo-chef 准备阶段
FROM rust:1.90-alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
WORKDIR /app

# 分析依赖
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# 构建阶段
FROM chef AS builder

# 安装构建依赖 - Alpine 自带 musl
RUN apk add --no-cache \
    pkgconf \
    openssl-dev \
    openssl-libs-static \
    libssh2-dev \
    libssh2-static \
    zlib-dev \
    zlib-static \
    musl-dev \
    perl \
    make \
    ca-certificates

# 设置编译选项
ENV RUSTFLAGS="-C link-arg=-s -C opt-level=z"

# 先编译依赖（这一层会被缓存）
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 再编译项目代码
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# 运行阶段 - 使用scratch
FROM scratch

LABEL maintainer="AptS:1547 <apts-1547@esaps.net>"
LABEL description="A tool to update your GitHub README with WakaTime stats."
LABEL version="1.0.0"
LABEL homepage="https://github.com/AptS-1547/wakatime-readme-updater"
LABEL license="MIT"

# 从构建阶段复制 CA 证书
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/wakatime-readme-updater /wakatime-readme-updater

# 设置环境变量
ENV DOCKER_ENV=1
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# 设置工作目录
WORKDIR /repo

# 启动命令
ENTRYPOINT ["/wakatime-readme-updater"]
