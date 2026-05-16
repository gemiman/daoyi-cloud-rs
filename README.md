# daoyi-cloud-rs

基于 Rust 的云原生微服务脚手架，采用 **Salvo + SeaORM + MySQL** 技术栈，支持**模块化单体**和**微服务（独立部署）**两种部署模式。

## 技术栈

| 类别     | 技术                               | 版本              |
|--------|----------------------------------|-----------------|
| Web 框架 | Salvo                            | 0.93            |
| 异步运行时  | Tokio                            | 1.52            |
| ORM    | SeaORM                           | 2.0-rc.38       |
| 数据库    | MySQL 8+                         | —               |
| API 文档 | salvo-oapi + Swagger UI + Scalar | 0.93            |
| 认证     | JWT (HS256) + RBAC               | jsonwebtoken 10 |
| 密码加密   | bcrypt                           | 0.19            |
| 配置管理   | config (YAML + 环境变量)             | 0.15            |
| 日志     | tracing + tracing-subscriber     | 0.1 / 0.3       |
| 参数校验   | validator                        | 0.20            |
| ID 生成  | 雪花算法 + XID                       | —               |
| 数据库迁移  | SeaORM Migration                 | 2.0-rc.38       |

## 项目结构

```
daoyi-cloud-rs/
├── src/                              # 主应用（聚合模式入口）
│   ├── main.rs                       # 聚合服务启动入口
│   └── api/mod.rs                    # 全局路由聚合
├── resources/                        # 配置文件
│   ├── application-server.yaml       # 聚合服务配置 (38080)
│   └── application-demo.yaml         # Demo 模块配置 (28080)
├── deploy/k8s/                       # Kubernetes 部署清单
│   ├── daoyi-module-demo.yaml        # 含 Deployment + HPA + Service + ConfigMap
│   ├── daoyi-cloud-rs.yaml           # 聚合服务 Deployment + Service (LoadBalancer)
│   └── daoyi-secrets.yaml            # Secret 模板
├── crates/
│   ├── libs/
│   │   ├── commons/daoyi-cloud-common/   # 公共基础设施库
│   │   └── entities/daoyi-entity-demo/   # 实体 + 模型 + 服务
│   ├── bins/
│   │   └── daoyi-module-demo/            # Demo API 模块（可独立部署）
│   └── migration/                        # SeaORM 数据库迁移
├── .github/workflows/ci.yml         # CI Pipeline
├── Dockerfile                        # 多阶段 Docker 构建
├── deny.toml                         # cargo-deny 依赖审计
├── CHANGELOG.md                      # 变更日志
└── .env.example                      # 环境变量模板
```

## 快速开始

### 环境要求

- Rust 1.94+
- MySQL 8+

### 初始化数据库

```shell
# 方式一：使用 SeaORM Migration（推荐）
cp .env.example .env                # 按需修改数据库连接信息
cargo run -p migration              # 自动建表 + 填充种子数据

# 方式二：手动导入 SQL
mysql -u root -p < docs/db/demo/schema.sql
```

### 配置 JWT 密钥

JWT Secret **不能为空**。通过环境变量配置：

```shell
export APP_AUTH_JWT_SECRET='your-256-bit-secret-here'
```

### 启动服务

```shell
# 聚合模式（所有模块，端口 38080）
RUST_LOG=info cargo run

# 独立模式（仅 Demo 模块，端口 28080）
RUST_LOG=info cargo run -p daoyi-module-demo

# JSON 日志格式（生产环境）
RUST_LOG=info RUST_LOG_FORMAT=json cargo run
```

### 验证服务

```shell
# 健康检查
curl http://localhost:38080/health

# 指标端点
curl http://localhost:38080/metrics

# API 文档
open http://localhost:38080/swagger-ui
```

## API 概览

### 认证

| 方法   | 路径                               | 说明                       |
|------|----------------------------------|--------------------------|
| POST | `/admin-api/demo/auth/login`     | 登录（返回 JWT）               |
| GET  | `/admin-api/demo/auth/user-info` | 获取当前用户信息（需 Bearer Token） |

### 用户管理（需 Bearer Token）

| 方法     | 路径                           | 说明      |
|--------|------------------------------|---------|
| GET    | `/admin-api/demo/users`      | 查询所有用户  |
| GET    | `/admin-api/demo/users/page` | 分页查询    |
| POST   | `/admin-api/demo/users`      | 创建用户    |
| PUT    | `/admin-api/demo/users/{id}` | 更新用户    |
| GET    | `/admin-api/demo/users/{id}` | 按 ID 查询 |
| DELETE | `/admin-api/demo/users/{id}` | 删除用户    |

## 架构设计

### 分层架构

```
API 层 (daoyi-module-demo)
  ├── Handlers（端点函数）
  ├── 参数提取/校验 (daoyi-cloud-common::extract)
  └── 统一响应 (ApiResponse / ApiError)
       │
服务层 (daoyi-entity-demo::service)
  ├── auth_service     (登录认证)
  └── sys_user_service (用户 CRUD)
       │
实体层 (daoyi-entity-demo::entity)
  └── sys_user (SeaORM Entity)
       │
基础设施层 (daoyi-cloud-common)
  ├── context  (AppContext + 依赖注入)
  ├── auth     (JWT + RBAC + Token 黑名单)
  ├── conf     (YAML + 环境变量)
  ├── cache    (内存缓存)
  ├── db       (MySQL 连接池，参数可配置)
  ├── error    (ApiError + ErrorCode + 自动日志)
  ├── response (统一响应格式)
  ├── server   (CORS / Metrics / 限流 / 超时 / 安全头)
  ├── logger   (文本/JSON 双模式)
  └── utils    (雪花ID / 密码 / 计时 / 序列化)
```

### 依赖注入（DI）模式

AppContext 通过 InjectContext 中间件注入到每个请求的 Depot：

```rust
// handler 中获取
let ctx = AppContext::from_depot(depot).unwrap();
let users = my_service::query_users(&ctx.db, params).await?;

// 旧代码全局访问（向后兼容）
db::get();
conf::get();
jwt::default_jwt();
```

### 双模式部署

- **聚合模式**：`cargo run` → 加载 `application-server.yaml` (38080)，聚合所有模块路由
- **独立模式**：`cargo run -p daoyi-module-demo` → 加载 `application-demo.yaml` (28080)

### 中间件链（外层 → 内层）

```
InjectCtx → RateLimit → SecurityHeaders → Metrics → RequestId → CORS → Timeout → Router → Handler
```

| 中间件             | 功能                                                         | 可配置                                         |
|-----------------|------------------------------------------------------------|---------------------------------------------|
| InjectCtx       | 注入 AppContext 到 Depot                                      | —                                           |
| RateLimit       | 固定窗口限流（默认 1000/min）                                        | `RATE_LIMIT_MAX` / `RATE_LIMIT_WINDOW_SECS` |
| SecurityHeaders | X-Content-Type-Options / X-Frame-Options / XSS 防护 等 6 个安全头 | —                                           |
| Metrics         | 请求计数 / 活跃数 / 4xx / 5xx                                     | —                                           |
| RequestId       | x-request-id 请求追踪                                          | —                                           |
| CORS            | 跨域资源共享                                                     | `server.cors` YAML 配置                       |
| Timeout         | 全局请求超时（30s）                                                | —                                           |

### 统一响应格式

```json
{
  "code": 0,
  "msg": "操作成功",
  "data": {},
  "requestId": "xxx"
}
```

- `code = 0` 表示成功，非 0 为业务错误码
- 业务错误码范围：1000~9999
- 错误自动按级别记录日志（5xx→error, 4xx→warn, 业务→info）

### OpenAPI 安全方案

| 方案          | 类型           | 请求头                             | 说明    |
|-------------|--------------|---------------------------------|-------|
| bearer_auth | Http(Bearer) | `Authorization: Bearer <token>` | JWT   |
| tenant_id   | ApiKey       | `tenant-id`                     | 租户 ID |

## 配置参考

### YAML 配置 (`resources/application-*.yaml`)

```yaml
server:
  port: 38080                      # 服务端口
  cors:
    allowed_origins: []            # 允许的域名，空=允许所有
    allowed_methods: []            # 允许的 HTTP 方法
    allowed_headers: []            # 允许的请求头
    allow_credentials: false       # 是否允许凭证
    max_age_secs: 43200            # 预检请求缓存时间
  tls:
    enabled: false                 # HTTPS 开关
    cert_path: ""                  # 证书路径
    key_path: ""                   # 私钥路径

database:
  host: 127.0.0.1
  port: 3306
  user: root
  password: 123456                 # 可通过 APP_DATABASE_PASSWORD 覆盖
  database: demo
  pool:
    min_connections: 2             # 最小连接数
    max_connections: 10            # 最大连接数
    connect_timeout_secs: 30
    idle_timeout_secs: 60
    max_lifetime_secs: 300

auth:
  ignore_urls:
    - "**/login"
    - "**/health"
  jwt:
    secret: ""                     # 必须通过环境变量 APP_AUTH_JWT_SECRET 设置
    expiration_secs: 3600          # Token 过期时间
    audience: "daoyi-cloud"
    issuer: "daoyi-cloud"
```

### 环境变量

参考 `.env.example`：

| 变量                       | 说明                     | 必填    |
|--------------------------|------------------------|-------|
| `APP_AUTH_JWT_SECRET`    | JWT 签名密钥               | **是** |
| `APP_DATABASE_PASSWORD`  | 数据库密码                  | 推荐    |
| `RUST_LOG`               | 日志级别 (info/debug/warn) | 否     |
| `RUST_LOG_FORMAT`        | 日志格式 (text/json)       | 否     |
| `RATE_LIMIT_MAX`         | 限流最大请求数                | 否     |
| `RATE_LIMIT_WINDOW_SECS` | 限流窗口（秒）                | 否     |
| `APP_CONFIG_PATH`        | 配置文件路径                 | 否     |

## 生产环境部署

### Docker 构建

```shell
docker build -t daoyi-cloud/daoyi-cloud-rs:latest .
docker run -d \
  -p 38080:38080 \
  -e APP_AUTH_JWT_SECRET='your-secret' \
  daoyi-cloud/daoyi-cloud-rs:latest
```

### Docker Compose

```yaml
services:
  app:
    build: .
    ports:
      - "38080:38080"
    environment:
      - APP_AUTH_JWT_SECRET=your-secret
      - APP_DATABASE_HOST=mysql
    depends_on:
      - mysql
  mysql:
    image: mysql:8
    environment:
      MYSQL_ROOT_PASSWORD: 123456
      MYSQL_DATABASE: demo
```

### Kubernetes

```shell
# 创建 Secret（请替换为实际值）
kubectl create secret generic daoyi-secrets \
  --from-literal=jwt_secret='your-secret' \
  --from-literal=db_password='your-password'

# 部署模块
kubectl apply -f deploy/k8s/daoyi-module-demo.yaml
kubectl apply -f deploy/k8s/daoyi-cloud-rs.yaml
```

### Release Profile

Cargo.toml 已内置优化配置：`opt-level=3` + `lto=true` + `codegen-units=1` + `strip=true` + `panic="abort"`。

```shell
cargo build --release
RUST_LOG=info RUST_LOG_FORMAT=json ./target/release/daoyi-cloud-rs
```

## 开发指南

### 生成 SeaORM Entity

```shell
cargo install sea-orm-cli@^2.0.0-rc
cd crates/libs/entities/daoyi-entity-demo
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity
```

### 创建数据库迁移

```shell
# 在 crates/migration 中添加新的迁移文件
# 参考 m20260516_000001_create_sys_user.rs
# 然后在 lib.rs 的 migrations() 列表中注册
```

### 创建新模块

1. 新建 `crates/bins/daoyi-module-xxx/`，实现 `create_router()`
2. 模块路由注册到 `src/api/mod.rs`
3. 独立运行：`cargo run -p daoyi-module-xxx`

### 运行测试

```shell
cargo test -p daoyi-entity-demo          # 运行 Service 层单元测试
cargo test -p daoyi-cloud-common         # 运行公共库测试
cargo clippy --workspace -- -D warnings  # Lint 检查
```

## 优化状态

项目经过 **8 轮共 50+ 项优化**，覆盖：

| 类别   | 完成项                                                     |
|------|---------------------------------------------------------|
| ✅ P0 | CI/Docker/迁移/JWT安全/健康检查/Metrics/DI/单元测试                 |
| ✅ P1 | 错误码/自动日志/RequestId/密码强度/CORS/RBAC/TLS/密码安全/连接池          |
| ✅ P2 | Profile/JSON日志/安全头/缓存/Changelog/OpenAPI/慢查询日志/种子数据/deny |
| ⬜    | 领域层分离 / 微服务gRPC / 读写分离 / WebSocket                      |

## License

MIT
