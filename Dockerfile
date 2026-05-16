# ===== Builder Stage =====
# 注意：版本必须与 Cargo.toml 中 rust-version 一致
FROM rust:1.94-slim-bookworm AS builder

WORKDIR /app

# 1. 先只复制依赖清单，利用 Docker 层缓存
COPY Cargo.toml Cargo.lock ./
COPY crates/libs/commons/daoyi-cloud-common/Cargo.toml crates/libs/commons/daoyi-cloud-common/
COPY crates/libs/entities/daoyi-entity-demo/Cargo.toml crates/libs/entities/daoyi-entity-demo/
COPY crates/bins/daoyi-module-demo/Cargo.toml crates/bins/daoyi-module-demo/
COPY crates/migration/Cargo.toml crates/migration/

# 2. 创建空 src 使 cargo 能解析依赖（构建时会跳过）
RUN mkdir -p crates/{libs/{commons/daoyi-cloud-common/src,entities/daoyi-entity-demo/src},bins/daoyi-module-demo/src,migration/src} \
    && touch crates/libs/commons/daoyi-cloud-common/src/lib.rs \
           crates/libs/entities/daoyi-entity-demo/src/lib.rs \
           crates/bins/daoyi-module-demo/src/lib.rs \
           crates/migration/src/lib.rs

# 3. 构建依赖（利用层缓存，除非 Cargo.toml/lock 变化）
RUN cargo build --release 2>/dev/null || true

# 4. 复制完整源码并重新构建
COPY . .
RUN cargo build --release && \
    cargo build --release -p daoyi-module-demo && \
    strip target/release/daoyi-cloud-rs && \
    strip target/release/daoyi-module-demo

# ===== Runtime Stage =====
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd -r daoyi && useradd -r -g daoyi daoyi

WORKDIR /app

COPY --from=builder /app/target/release/daoyi-cloud-rs ./
COPY --from=builder /app/target/release/daoyi-module-demo ./
COPY --from=builder /app/resources/ ./resources/

RUN chown -R daoyi:daoyi /app

USER daoyi

EXPOSE 38080 28080

ENTRYPOINT ["./daoyi-cloud-rs"]

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -sf http://localhost:38080/health || curl -sf http://localhost:28080/health || exit 1
