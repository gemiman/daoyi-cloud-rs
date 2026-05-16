# ===== Builder Stage =====
FROM rust:1.84-slim-bookworm AS builder

WORKDIR /app

# 先复制依赖清单，利用 Docker 层缓存加速构建
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY src/ ./src/

# 构建发布版本（默认聚合模式）
RUN cargo build --release && \
    # 构建独立模块模式
    cargo build --release -p daoyi-module-demo && \
    # 瘦身：移除调试符号
    strip target/release/daoyi-cloud-rs && \
    strip target/release/daoyi-module-demo

# ===== Runtime Stage =====
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN groupadd -r daoyi && useradd -r -g daoyi daoyi

WORKDIR /app

COPY --from=builder /app/target/release/daoyi-cloud-rs ./
COPY --from=builder /app/target/release/daoyi-module-demo ./
COPY --from=builder /app/resources/ ./resources/

RUN chown -R daoyi:daoyi /app

USER daoyi

EXPOSE 38080
EXPOSE 28080

# 默认启动聚合服务，可通过 docker run --entrypoint 覆盖
ENTRYPOINT ["./daoyi-cloud-rs"]

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:38080/health || curl -f http://localhost:28080/health || exit 1
