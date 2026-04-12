# daoyi-cloud-rs

基于 Rust 的云原生微服务脚手架，采用 Axum + SeaORM + PostgreSQL 技术栈，支持模块化单体与微服务两种部署模式。

## 技术栈

| 类别     | 技术                           | 版本              |
|--------|------------------------------|-----------------|
| Web 框架 | Axum                         | 0.8             |
| 异步运行时  | Tokio                        | 1               |
| ORM    | SeaORM                       | 2.0             |
| 数据库    | PostgreSQL                   | -               |
| API 文档 | utoipa + Swagger UI + Scalar | 5 / 9 / 0.3     |
| 认证     | JWT (HS256)                  | jsonwebtoken 10 |
| 密码加密   | bcrypt                       | 0.19            |
| 配置管理   | config (YAML + 环境变量)         | 0.15            |
| 日志     | tracing + tracing-subscriber | 0.1 / 0.3       |
| 参数校验   | validator + axum-valid       | 0.20 / 0.24     |
| ID 生成  | 雪花算法 + XID                   | -               |

## 项目结构

```
daoyi-cloud-rs/                      # 主应用（聚合模式入口）
├── src/
│   ├── main.rs                      # 聚合服务启动入口
│   └── api/mod.rs                   # 全局路由聚合
├── resources/
│   ├── application-server.yaml      # 聚合服务配置（端口 38080）
│   └── application-demo.yaml        # Demo 模块配置（端口 28080）
├── docs/db/demo/
│   └── schema.sql                   # 数据库 Schema
└── crates/
    ├── libs/
    │   ├── commons/daoyi-cloud-common   # 公共基础库
    │   └── entities/daoyi-entity-demo   # 实体 + 模型 + 服务
    └── bins/
        └── daoyi-module-demo            # Demo API 模块（可独立部署）
```

## 快速开始

### 环境要求

- Rust 1.94+
- PostgreSQL

### 初始化数据库

```shell
# 执行 Schema 初始化
psql -U postgres -f docs/db/demo/schema.sql
```

### 启动服务

```shell
# 聚合模式（所有模块，端口 38080）
RUST_LOG=DEBUG cargo run

# 独立模式（仅 Demo 模块，端口 28080）
RUST_LOG=DEBUG cargo run -p daoyi-module-demo
```

### API 文档

启动后访问：

| UI         | 地址                                    |
|------------|---------------------------------------|
| Swagger UI | `http://localhost:{port}/swagger-ui/` |
| Scalar     | `http://localhost:{port}/scalar`      |

## 开发指南

### 生成 Entity

```shell
# 安装 sea-orm-cli
cargo install sea-orm-cli@^2.0.0-rc

# 生成 entity（在 daoyi-entity-demo 目录下执行）
cd crates/libs/entities/daoyi-entity-demo
sea-orm-cli generate entity \
  -s demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity
```

### 升级依赖

```shell
# 安装 cargo-edit
cargo install cargo-edit

# 预览升级
cargo upgrade --dry-run

# 兼容性升级
cargo upgrade

# 不兼容升级（跨大版本）
cargo upgrade -i

# 指定包升级
cargo upgrade -p axum

# 排除包
cargo upgrade -i --exclude sea-orm
```

## 架构设计

### 双模式部署

- **聚合模式**：主应用加载所有业务模块的路由，统一启动，适合开发和小规模部署
- **独立模式**：每个业务模块是独立的 bin crate，可单独部署为微服务

### 请求处理流程

```
Request → CORS → Tracing → Timeout → BodyLimit → NormalizePath → JWT Auth → Handler → Response
```

### 统一响应格式

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

- `code = 0` 表示成功
- `code != 0` 表示失败，`msg` 为错误信息

## Git 推送配置

同时推送到 Gitee 和 GitHub：

```gitconfig
[remote "origin"]
    url = git@gitee.com:daoyi2026/daoyi-cloud-rs.git
    pushurl = git@gitee.com:daoyi2026/daoyi-cloud-rs.git
    pushurl = git@github.com:gemiman/daoyi-cloud-rs.git
```

```shell
git push origin master   # 推送到 Gitee + GitHub
```

## 生产环境构建

```shell
# 构建 release 版本（启用 LTO + strip + panic=abort，体积最小）
cargo build --release

# 运行
RUST_LOG=INFO ./target/release/daoyi-cloud-rs
RUST_LOG=INFO ./target/release/daoyi-module-demo
```

Release Profile 配置（已内置在 Cargo.toml）：

| 配置项             | 值     | 说明                 |
|-----------------|-------|--------------------|
| `opt-level`     | 3     | 最高优化等级             |
| `lto`           | true  | 跨 crate 链接时优化，减小体积 |
| `codegen-units` | 1     | 单编译单元，更优的优化效果      |
| `strip`         | true  | 去除调试符号，减小体积        |
| `panic`         | abort | 直接终止而非 unwind，减小体积 |

## 发布到 crates.io

本项目中有两个 lib crate 可发布到 crates.io，bin crate 已设置 `publish = false` 不参与发布。

### 依赖关系与发布顺序

```
daoyi-cloud-common  ←  daoyi-entity-demo
     ①                      ②
```

必须先发布 `daoyi-cloud-common`，再发布 `daoyi-entity-demo`（后者依赖前者）。

### 1. 登录 crates.io

```shell
# 方式一：使用 API Token（推荐）
cargo login <your-api-token>
# Token 获取：https://crates.io/settings/tokens

# 方式二：使用 GitHub OAuth
cargo login
# 会打开浏览器进行 GitHub 授权
```

### 2. 检查待发布包

```shell
# 检查 package 元数据和打包内容
cd crates/libs/commons/daoyi-cloud-common
cargo publish --dry-run

cd crates/libs/entities/daoyi-entity-demo
cargo publish --dry-run
```

### 3. 发布

```shell
# 第一步：发布 daoyi-cloud-common
cd crates/libs/commons/daoyi-cloud-common
cargo publish

# 等待 crates.io 索引同步（通常几秒到几分钟）
# 可通过 https://crates.io/crates/daoyi-cloud-common 确认

# 第二步：发布 daoyi-entity-demo
cd crates/libs/entities/daoyi-entity-demo
cargo publish
```

### 4. 版本更新流程

```shell
# 1. 修改 workspace.package.version（所有 crate 版本同步更新）
#    编辑根 Cargo.toml 的 [workspace.package] version
#    同时更新 [workspace.dependencies] 中的 path 依赖版本号

# 2. 提交并打 tag
git add -A
git commit -m "release v0.9.1"
git tag v0.9.1
git push origin master --tags

# 3. 按顺序发布
cd crates/libs/commons/daoyi-cloud-common && cargo publish
cd crates/libs/entities/daoyi-entity-demo && cargo publish
```

### 5. 撤回已发布版本

```shell
# crates.io 允许在发布 72 小时内撤回（yank），已 yank 的版本仍可使用但不推荐
cargo yank --vers 0.9.0 daoyi-cloud-common
cargo yank --vers 0.9.0 daoyi-entity-demo

# 取消撤回
cargo yank --vers 0.9.0 --undo daoyi-cloud-common
```

### 注意事项

- crates.io 不允许覆盖已发布的版本号，每次发布必须使用新版本号
- `license` 字段必须设置（已配置为 MIT）
- `description` 字段不能为空（已配置）
- 发布前确保 `cargo test` 和 `cargo clippy` 通过
- workspace 中 `license-file = "LICENSE"` 仅对根包生效，lib crate 使用 `license = "MIT"`
