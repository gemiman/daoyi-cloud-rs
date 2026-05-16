# daoyi-cloud-rs 项目优化指南

> 基于 Rust 微服务最佳实践与生产环境要求的全面分析
> 评估日期：2026-05-16

---

## 目录

1. [架构设计](#1-架构设计)
2. [配置管理](#2-配置管理)
3. [数据库层](#3-数据库层)
4. [错误处理](#4-错误处理)
5. [认证与授权](#5-认证与授权)
6. [API 层](#6-api-层)
7. [可观测性](#7-可观测性)
8. [测试策略](#8-测试策略)
9. [安全加固](#9-安全加固)
10. [性能优化](#10-性能优化)
11. [DevOps 与 CI/CD](#11-devops-与-cicd)
12. [代码质量](#12-代码质量)
13. [依赖管理](#13-依赖管理)

---

## 1. 架构设计

### 1.1 全局静态单例（`OnceCell`/`Lazy`）泛滥

**问题描述**：`AppConfig`、`DatabaseConnection`、`JWT` 实例全部使用全局 `OnceCell` 初始化，通过 `get()` 静态方法访问。这导致：

- **单元测试困难**：无法 mock 配置、数据库、JWT，侧写测试时无法替换依赖。
- **并发安全隐患**：全局可变状态违反 `Send + Sync` 最佳实践。
- **初始化顺序耦合**：`app::run()` 必须按固定顺序初始化全局变量，若某一步失败，后续模块仍可访问未初始化的单例引发 panic。
- **无法并行初始化**：`OnceCell::get_or_init` 是同步的，不能跟 async 初始化组合。

**优化建议**：

```
// 当前：全局 + OnceCell
pub fn get() -> &'static AppConfig { ... }

// 优化：使用依赖注入 / 结构体封装上下文
pub struct AppContext {
    pub config: AppConfig,
    pub db: DatabaseConnection,
    pub jwt: JWT,
}

pub async fn build_context(app_name: &str) -> Result<AppContext> {
    let config = AppConfig::load(app_name)?;
    let db = db::connect(&config.database).await?;
    let jwt = JWT::new(&config.auth.jwt)?;
    Ok(AppContext { config, db, jwt })
}

// handler 通过 extractor 获取 AppContext
async fn handler(ctx: extract::AppContext) -> Json<ApiResponse<...>> { ... }
```

**优先级**：P0（高）

### 1.2 缺少真正的领域层

**问题描述**：`daoyi-entity-demo` 混用了 SeaORM Entity、DTO（Models）、Service（业务逻辑）。这违反了"领域驱动设计"的分层原则：

- Entity 层与 Service 层耦合在同一个 crate 中，无法独立演进。
- Service 通过 `crate::demo::entity::*` 直接访问 DB Entity，无法运行单元测试。
- Models 既是请求参数又是响应模型，同一结构体不可能同时满足两者需求（如创建时不需要 `id`、`created_at`，但查询时需要）。

**优化建议**：

```
crates/
├── domain/                    # 领域层
│   └── daoyi-demo-domain/     # 纯业务逻辑，无依赖框架
│       └── src/
│           ├── service/       # 业务服务（trait）
│           ├── model/         # 领域模型
│           └── port/          # 输出端口（Repository trait）
├── infrastructure/            # 基础设施层
│   └── daoyi-demo-persistence/
│       ├── entity/            # SeaORM Entity
│       ├── repository/        # 实现 port 中的 trait
│       └── migration/         # 数据库迁移
└── api/                       # API 层（DTO 独立）
    └── daoyi-demo-api/
        └── dto/               # 请求/响应 DTO，与领域模型分离
```

**优先级**：P0（高）

### 1.3 微服务边界不清晰

**问题描述**：

- 所有模块通过 `Router::new().push(...)` 聚合，模块间无隔离。
- 没有强制模块间的服务边界（如 gRPC 接口或 REST 调用）。
- "聚合模式"和"独立模式"只是编译参数不同，没有真正的运行时服务发现。

**优化建议**：

```
// 1. 定义跨模块通信接口（gRPC 或 HTTP 客户端）
// 模块 A -> (gRPC client) -> 模块 B

// 2. 模块间通过服务发现地址访问
// 3. 聚合模式只能用于开发环境，生产环境应使用独立部署
```

**优先级**：P2（中）

### 1.4 缺乏熔断与重试机制

**问题描述**：服务间调用（如聚合模式下的模块间通信）无熔断、重试、超时控制。

**优化建议**：引入 `tokio::time::timeout` 包装外部调用，或使用 `futures-retry`/`sabre`（Rust 版 Hystrix）。

**优先级**：P2（中）

---

## 2. 配置管理

### 2.1 配置文件路径硬编码

**问题描述**：`resources/application-{app_name}.yaml` 路径在 `conf/mod.rs`
中硬编码，无法通过环境变量指定自定义路径。不同环境（dev/staging/prod）需复制文件。

**优化建议**：

```rust
// 当前
let config_path = format!("resources/application-{}.yaml", app_name);

// 优化：优先从环境变量读，其次走默认值
let config_path = std::env::var("APP_CONFIG_PATH")
.unwrap_or_else( | _ | format!("resources/application-{}.yaml", app_name));
```

**优先级**：P1（中高）

### 2.2 数据库密码明文存储

**问题描述**：`application-*.yaml` 中数据库密码为明文。生产环境应使用加密存储或密钥管理服务。

**优化建议**：

```
方案 A：环境变量覆盖
  database.password: ${DB_PASSWORD}   # config crate 支持

方案 B：Vault/HashiCorp
  从 K8s Secret / AWS Secret Manager / HashiCorp Vault 读取

方案 C：`.env` 文件
  使用 dotenvy 加载 .env 文件（已有 .env 文件但未使用）
```

**优先级**：P1（中高）

### 2.3 JWT Secret 硬编码默认值

**问题描述**：`DEFAULT_JWT_SECRET` 在代码中硬编码。若有默认值，攻击者可直接伪造 JWT。

**优化建议**：

```rust
// 移除默认值
pub struct JwtConfig {
    pub secret: String, // 必须配置，无默认值
}

// 启动时校验
pub fn load() -> Result<AppConfig> {
    if config.auth.jwt.secret.is_empty() {
        return Err(anyhow!("JWT secret must be configured"));
    }
    // ...
}
```

**优先级**：P0（高）

### 2.4 配置缺少 Profile 支持

**问题描述**：当前仅支持单文件配置，不支持 Spring Boot 风格的 profile 覆盖机制（如 `application-dev.yaml` 覆盖
`application.yaml`）。

**优化建议**：支持多层配置覆盖：

```
application.yaml       # 公共默认值
application-dev.yaml   # 开发环境覆盖
application-prod.yaml  # 生产环境覆盖
```

**优先级**：P2（中）

---

## 3. 数据库层

### 3.1 缺少 Schema 迁移工具

**问题描述**：当前 `docs/db/demo/schema.sql` 是手动维护的 SQL 文件，不跟随代码版本自动执行。`ddl.sql` 甚至为空文件。

- Schema 变更需要手动同步到多个环境。
- 无法回滚。
- 新人入职需手动执行 SQL。

**优化建议**：

```toml
# 方案 A：使用 SeaORM Migration
sea-orm-migration = { version = "2.0", features = ["sqlx-mysql"] }

# 方案 B：使用 sqlx::migrate!
# 将 SQL 放在 migrations/ 目录下，编译时校验
sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
```

**优先级**：P0（高）

### 3.2 缺少读写分离

**问题描述**：所有查询和写入使用同一个数据库连接池。对于读多写少的微服务，应分离读库和写库。

**优化建议**：

```rust
pub struct DatabasePool {
    pub writer: DatabaseConnection,  // 主库
    pub reader: DatabaseConnection,  // 从库
}

// service 中显式区分
async fn get_user(db: &DatabasePool, id: i64) -> Result<Model> {
    db.reader.find(Model::by_id(id)).await?
}

async fn create_user(db: &DatabasePool, params) -> Result<Model> {
    db.writer.save(active_model).await?
}
```

**优先级**：P2（中）

### 3.3 连接池参数不可配置

**问题描述**：最大/最小连接数、超时时间在代码中硬编码，未在配置文件中暴露。

**优化建议**：

```yaml
database:
  host: 127.0.0.1
  port: 3306
  pool:
    min_connections: 2
    max_connections: 10
    connect_timeout_secs: 30
    idle_timeout_secs: 60
    max_lifetime_secs: 300
    acquire_timeout_secs: 30
```

**优先级**：P1（中高）

### 3.4 缺少连接池健康检查

**问题描述**：没有定期检查数据库连接健康状态的机制。长连接可能在 MySQL 空闲超时后被 KILL，但 SeaORM 连接池不知道。

**优化建议**：启用 SeaORM/SQLx 的 `test_before_acquire` 选项，或在启动时运行健康检查协程。

**优先级**：P1（中高）

### 3.5 缺少数据库查询日志

**问题描述**：没有记录慢查询日志、SQL 执行时长、影响行数等指标。

**优化建议**：

```rust
// 使用 SeaORM 的 DebugQuery 或自定义中间件
use sea_orm::DebugQuery;

// 或在 service 层手动包裹计时
let start = std::time::Instant::now();
let result = query.exec(db).await?;
tracing::debug!("SQL executed in {:?}", start.elapsed());
```

**优先级**：P2（中）

### 3.6 种子数据不宜放在 SQL 文件中

**问题描述**：`schema.sql` 中包含种子数据 INSERT 语句，应与 Schema DDL 分离为独立迁移步骤。

**优化建议**：迁移文件只负责 Schema 变更，种子数据应通过代码或独立数据初始化脚本管理。

**优先级**：P2（中）

---

## 4. 错误处理

### 4.1 ApiError 映射过多、缺乏业务语义

**问题描述**：`ApiError` 直接映射到 HTTP StatusCode，但缺少业务错误码。例如"用户已禁用"(1001)和"密码错误"(1002)都返回
401，前端无法区分。

**优化建议**：

```rust
pub struct ApiError {
    pub code: i32,       // 业务错误码 (1001, 1002, ...)
    pub msg: String,     // 用户友好提示
    pub status: StatusCode,
    pub cause: Option<anyhow::Error>, // 内部错误（不暴露给客户端）
}
```

**优先级**：P1（中高）

### 4.2 所有错误都记录在 API 层

**问题描述**：当前的错误处理未记录错误日志。生产环境中，500 错误应记录完整堆栈，但不应暴露给客户端。

**优化建议**：

```rust
impl From<DbErr> for ApiError {
    fn from(e: DbErr) -> Self {
        tracing::error!(error = %e, "Database error occurred");
        ApiError {
            code: 5000,
            msg: "内部服务错误".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
            cause: Some(e.into()),
        }
    }
}
```

**优先级**：P1（中高）

### 4.3 缺少 RequestId 追踪

**问题描述**：无请求级别的唯一 ID，报错时无法将日志 & API 响应关联。

**优化建议**：

```rust
// 1. 添加中间件生成 RequestId
pub struct RequestIdMiddleware;

impl Handler for RequestIdMiddleware {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, _res: &mut Response) {
        let request_id = xid();  // 或 UUID
        depot.insert("request_id", request_id.clone());
        req.headers_mut().insert("X-Request-Id", request_id.parse().unwrap());
        // 注入 tracing span
        let span = info_span!("request", id = %request_id);
        let _guard = span.enter();
    }
}
```

**优先级**：P1（中高）

---

## 5. 认证与授权

### 5.1 无 Role-Based Access Control（RBAC）

**问题描述**：JWT 仅包含 `tenant_id:id:name`，缺少角色和权限信息。所有已认证用户默认拥有全部权限，无法做细粒度授权。

**优化建议**：

```rust
pub struct Principal {
    pub tenant_id: i64,
    pub id: i64,
    pub name: String,
    pub roles: Vec<String>,       // ["admin", "user"]
    pub permissions: Vec<String>, // ["user:create", "user:delete"]
}
```

**优先级**：P1（中高）

### 5.2 无 Refresh Token

**问题描述**：Access Token 过期后用户必须重新登录。没有 Refresh Token 机制，用户体验差。

**优化建议**：

```rust
pub struct TokenPair {
    pub access_token: String,   // 短期（15分钟）
    pub refresh_token: String,  // 长期（7天）
}

// POST /auth/refresh
pub async fn refresh_token(params: RefreshParams) -> ApiResult<TokenPair>;
```

**优先级**：P1（中高）

### 5.3 JWT 默认密钥已知

**问题描述**：`DEFAULT_JWT_SECRET = "1qasrf45Xt6yh45tyhj6Q7yuikl89iolfty7"` 是公开的代码。若用户未覆盖该配置，任何人都可伪造
JWT。

**优化建议**：

```rust
// 方案 A：移除默认值，启动时强制校验
// 方案 B：随机生成密钥并在首次启动时打印到日志
// 方案 C：使用 RSA/ECDSA 非对称加密，私钥从未写入代码
```

**优先级**：P0（高）

### 5.4 Token 无状态失效机制

**问题描述**：JWT 签发后无法主动失效（除非更改密钥）。当用户注销/被踢出时，已签发的 JWT 仍有效直到过期。

**优化建议**：

```rust
// 方案 A：维护 Redis 黑名单 (token jti blacklist)
// 方案 B：使用 Refresh Token 在服务端维护状态

pub async fn logout(principal: &Principal, jti: &str) -> ApiResult<()> {
    // 将 jti 加入 Redis 黑名单，剩余有效期
    cache.set_ex(format!("blacklist:jwt:{}", jti), "1", remaining_ttl).await?;
}
```

**优先级**：P1（中高）

### 5.5 缺少 Token 续签防并发机制

**问题描述**：多客户端并发请求 Refresh 时可能多次刷新 Token，导致旧 Token 仍可用。

**优化建议**：Refresh Token 支持轮换（Rotation）——刷新 Token 后旧 Refresh Token 立即失效。

**优先级**：P2（中）

---

## 6. API 层

### 6.1 缺少 API 版本管理

**问题描述**：API 路径硬编码为 `/admin-api/demo/...`，没有版本控制（如 `/v1/admin-api/...`）。API 变更前后向兼容困难。

**优化建议**：

```rust
// 方式1：URL Path 版本
/admin-api/v1/demo/users
/admin-api/v2/demo/users

// 方式2：Header 版本
Accept: application/vnd.daoyi.v1+json
```

**优先级**：P2（中）

### 6.2 缺少请求限流

**问题描述**：没有限流中间件，恶意用户可大量请求压垮服务。

**优化建议**：

```rust
// 基于令牌桶的限流
use governor::{Quota, RateLimiter};
use nonzero_ext::*;

pub struct RateLimitMiddleware {
    limiter: RateLimiter<NotKeyed>,
}

impl Handler for RateLimitMiddleware {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self.limiter.check() {
            Ok(_) => { /* 继续 */ }
            Err(_) => { /* 429 Too Many Requests */ }
        }
    }
}
```

推荐库：`governor`（基于 GCRA 算法）、`tower::limit`。

**优先级**：P1（中高）

### 6.3 API 响应未包含请求追踪 ID

**问题描述**：`ApiResponse` 中没有 `request_id` 字段，客户端无法将请求与服务器日志关联。

**优化建议**：

```rust
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub request_id: String,  // 新增
    pub data: Option<T>,
}
```

**优先级**：P1（中高）

### 6.4 缺少 WebSocket / SSE 支持

**问题描述**：架构中未预留实时通信（WebSocket / Server-Sent Events）通道。对于需要实时推送的场景（通知、消息），需要事后改造。

**优化建议**：在路由架构中预留 WebSocket 路径，或考虑引入 `tokio-tungstenite`。

**优先级**：P3（低）

### 6.5 缺少全局请求超时

**问题描述**：没有默认的请求处理超时。若某个 handler 卡死（如数据库 hang），连接将长时间占用。

**优化建议**：

```rust
// Salvo 支持超时
use salvo::catcher::TimeOut;

let service = Service::new(router)
    .hoop(TimeOut::new(std::time::Duration::from_secs(30)));
```

**优先级**：P1（中高）

---

## 7. 可观测性

### 7.1 缺少 Metrics / Prometheus

**问题描述**：没有任何度量指标（请求 QPS、P99/P95 延迟、错误率）。无法做生产监控和告警。

**优化建议**：

```toml
# Cargo.toml
salvo = { version = "0.93", features = ["prometheus"] }
```

```rust
// 添加 Prometheus 中间件
use salvo::extra::prometheus::Prometheus;

let metrics_handler = Prometheus::new();
let router = Router::new()
    .hoop(metrics_handler)
    .push(demo::create_router())
    .push(Router::with_path("/metrics").get(metrics_handler.report()));
```

推荐指标：

- **RED 指标**：Rate（请求率）、Errors（错误率）、Duration（延迟分布）
- **USE 指标**：Utilization、Saturation、Errors（系统资源）
- **业务指标**：活跃用户数、订单量等

**优先级**：P0（高）

### 7.2 缺少分布式追踪

**问题描述**：使用 `tracing` 进行日志记录，但未接 OpenTelemetry。无法将跨服务（例如认证服务 -> 用户服务）的请求链路串联起来。

**优化建议**：

```toml
opentelemetry = { version = "0.29", features = ["trace", "rt-tokio"] }
opentelemetry-otlp = "0.29"
tracing-opentelemetry = "0.32"
```

```rust
// 集成 tracing 与 OpenTelemetry
use opentelemetry_otlp::WithExportConfig;

fn init_tracer() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(EnvFilter::from_default_env())
            .finish()
    ).unwrap();
}
```

**优先级**：P1（中高）

### 7.3 缺少结构化日志最佳实践

**问题描述**：当前日志格式包含文件名、行号、线程信息，但缺少结构化字段（JSON 格式），不适合日志聚合（ELK/Loki）。

**优化建议**：提供 JSON 日志格式选项，通过环境变量切换：

```rust
fn init_logger(json: bool) {
    if json {
        // JSON 格式日志（生产环境）
        tracing_subscriber::fmt()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .init();
    } else {
        // 彩色文本格式（开发环境）
        tracing_subscriber::fmt()
            .with_line_number(true)
            .with_thread_ids(true)
            .init();
    }
}
```

**优先级**：P2（中）

### 7.4 缺少健康检查端点

**问题描述**：没有 `/health` / `/ready` 端点。Kubernetes 等容器编排平台依赖这些端点做存活检测（Liveness）、就绪检测（Readiness）和启动检测（Startup）。

**优化建议**：

```rust
// GET /health -> 返回服务运行状态
Router::with_path("health").get(|req: &mut Request| async move {
    // Liveness: 服务是否存活（简单）
    let liveness = true;

    // Readiness: 依赖是否准备好（数据库连接等）
    let db_ok = matches!(db::get().ping().await, Ok(()));

    let status_code = if liveness && db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Json(json!({
        "status": if status_code.is_success() { "UP" } else { "DOWN" },
        "checks": {
            "database": if db_ok { "UP" } else { "DOWN" },
        }
    }))
});
```

**优先级**：P0（高）

---

## 8. 测试策略

### 8.1 测试覆盖率严重不足

**问题描述**：全项目仅 `constants/global_values.rs` 有少量单元测试。

**缺失测试类型**：

| 测试类型          | 当前 | 目标         |
|---------------|----|------------|
| 单元测试（Service） | 0  | 覆盖率 > 80%  |
| 集成测试（API）     | 0  | 每个端点至少 1 个 |
| 数据库测试         | 0  | 关键 CRUD 路径 |
| 认证测试          | 0  | 正常/异常认证路径  |
| 性能/压力测试       | 0  | 上线前必须      |

**优化建议**：

```rust
// Service 层测试示例 (无需真实数据库)
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_with_invalid_password() {
        // 当 Service 通过 trait 依赖 DB 时，可 mock
        let mut mock_repo = MockUserRepository::new();
        mock_repo.expect_find_by_account()
            .returning(|_| Ok(Some(sample_user())));

        let service = AuthService::new(mock_repo);
        let result = service.login(LoginParams {
            account: "admin".into(),
            password: "wrong_password".into(),
        }).await;

        assert!(result.is_err());
    }
}
```

**优先级**：P0（高）

### 8.2 全局单例导致无法 Mock

**问题描述**：全局 `OnceCell` 变量导致无法在测试中插入 mock 对象。`db::get()` 和 `conf::get()` 在测试中无法替换。

**优化建议**：结合第 1.1 节，使用依赖注入（`AppContext`）代替全局单例，使测试可传 mock DB。

**优先级**：P0（高）

### 8.3 缺乏测试基础设施

**问题描述**：没有测试辅助库（如 `testcontainers` 启动 MySQL）、没有 bench 测试、没有模糊测试。

**优化建议**：

```rust
// testcontainers：测试中自动启动 MySQL
#[tokio::test]
async fn test_create_user_integration() {
    let docker = testcontainers::clients::Cli::default();
    let mysql = docker.run(images::mysql::Mysql::default());
    let db = connect_to_mysql(mysql.get_host(), mysql.get_host_port(3306)).await;
    // ... 执行测试 ...
}
```

**优先级**：P2（中）

---

## 9. 安全加固

### 9.1 HTTPS 支持缺失

**问题描述**：仅支持 HTTP，不支持 HTTPS/TLS。生产环境服务需对外暴露 HTTPS 接口。

**优化建议**：

```yaml
server:
  port: 443
  tls:
    enabled: true
    cert_path: /etc/certs/server.crt
    key_path: /etc/certs/server.key
```

可在上游代理（Nginx/ALB）层处理 TLS，也可以直接在 Salvo 中启用 TLS。

**优先级**：P1（中高）

### 9.2 密码强度不足

**问题描述**：密码最短长度仅 6 位（`LoginParams` 中 `length(min = 6)`），现代标准建议至少 8-12 位且要求复杂度。

**优化建议**：

```rust
pub struct LoginParams {
    #[validate(length(min = 8, max = 32))]
    pub account: String,
    #[validate(length(min = 8, max = 64), custom = "validate_password_strength")]
    pub password: String,
}
```

**优先级**：P1（中高）

### 9.3 缺少安全头

**问题描述**：响应缺少常见 HTTP 安全头，如 `X-Content-Type-Options: nosniff`、`X-Frame-Options: DENY`、
`Strict-Transport-Security` 等。

**优化建议**：

```rust
pub struct SecurityHeadersMiddleware;

impl Handler for SecurityHeadersMiddleware {
    async fn handle(&self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.headers_mut().insert(
            "X-Content-Type-Options", "nosniff".parse().unwrap()
        );
        res.headers_mut().insert(
            "X-Frame-Options", "DENY".parse().unwrap()
        );
        res.headers_mut().insert(
            "X-XSS-Protection", "1; mode=block".parse().unwrap()
        );
    }
}
```

**优先级**：P2（中）

### 9.4 CORS 配置过于宽松

**问题描述**：当前使用 `PermissiveCors` 允许所有来源。生产环境应限制到特定域名。

**优化建议**：

```yaml
server:
  cors:
    allowed_origins:
      - "https://admin.example.com"
      - "https://console.example.com"
    allowed_methods: ["GET", "POST", "PUT", "DELETE"]
    allowed_headers: ["Authorization", "Content-Type", "X-Tenant-Id"]
```

**优先级**：P1（中高）

---

## 10. 性能优化

### 10.1 未使用连接复用池

**问题描述**：虽然 SeaORM 连接池本身会复用连接，但缺少针对热点查询的结果缓存。

**优化建议**：对高频只读查询（如字典、配置）使用内存缓存（`moka` 或`cached` crate）：

```rust
use moka::sync::Cache;

static USER_CACHE: Lazy<Cache<i64, Model>> = Lazy::new(|| {
    Cache::builder()
        .max_capacity(1000)
        .time_to_live(Duration::from_secs(300))
        .build()
});

pub async fn get_user_by_id(db: &DbConn, id: i64) -> Result<Model> {
    if let Some(user) = USER_CACHE.get(&id) {
        return Ok(user);
    }
    let user = find_user_by_id(db, id).await?;
    USER_CACHE.insert(id, user.clone());
    Ok(user)
}
```

**优先级**：P2（中）

### 10.2 未使用连接池预热

**问题描述**：启动后第一个请求才触发数据库连接建立，首请求延迟较高。

**优化建议**：

```rust
pub async fn init() -> anyhow::Result<()> {
    // 预热：提前建立最小连接数
    connection_pool.ping().await?;

    // 预热连接池：同时建立 min_connections 个连接
    for _ in 0..pool_config.min_connections {
        tokio::spawn(async {
            let _ = connection_pool.acquire().await;
        });
    }
}
```

**优先级**：P2（中）

### 10.3 未对高频查询做批量优化

**问题描述**：`service/sys_user_service.rs` 中涉及批量查询的场景，但未使用 SeaORM 的 `find_many`、`find_with_related`
等批量操作。

**优化建议**：

- N+1 查询检测
- 使用 `Model::find().all()` 代替循环单条查询
- 使用 `lazy_static`/`once_cell` 缓存只读配置

**优先级**：P2（中）

### 10.4 无序列化/反序列化优化

**问题描述**：当前使用默认 `serde_json` 序列化，未考虑：

- JSON 字段命名约定（蛇形/CamelCase 混合）
- 可选字段序列化省略
- 大对象流式响应

**优化建议**：启用 `preserve_order` 或使用更快的序列化库（如 `simd-json` 替代 `serde_json`）。

**优先级**：P3（低）

---

## 11. DevOps 与 CI/CD

### 11.1 缺少 Docker 镜像构建配置

**问题描述**：没有 `Dockerfile` 或 Docker Compose 配置。

**优化建议**：

```dockerfile
# 多阶段构建 Dockerfile
FROM rust:1.84-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin daoyi-module-demo
```

**优先级**：P0（高）

### 11.2 缺少 CI 配置文件

**问题描述**：没有 `.github/workflows/` 或 `.gitlab-ci.yml` 等 CI 配置。

**优化建议**：

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

**优先级**：P0（高）

### 11.3 缺少 K8s 部署清单

**问题描述**：没有 Kubernetes Deployment / Service / ConfigMap 清单文件。

**优化建议**：创建 `deploy/k8s/` 目录，包含各模块的 Deployment、Service、HPA（Horizontal Pod Autoscaler）、PodDisruptionBudget
等清单。

**优先级**：P1（中高）

### 11.4 缺少版本发布策略

**问题描述**：项目版本号 `0.9.0`，但无 changelog 或版本发布脚本。

**优化建议**：

- 创建 `CHANGELOG.md` 遵循 Keep a Changelog 格式
- 使用 `cargo-release` 或 `release-plz` 自动化版本发布
- 遵循 SemVer 规范

**优先级**：P2（中）

---

## 12. 代码质量

### 12.1 宏过度使用

**问题描述**：`page_query_params!` 宏虽减少了样板代码，但带来了以下问题：

- 无法在 IDE 中跳转到宏展开代码
- 宏生成的文档不完整
- 调试困难（错误信息指向宏调用点而非展开代码）
- 新开发者需学习自定义 DSL

**优化建议**：

```rust
// 使用派生宏替代
#[derive(PageQuery)]
pub struct UserQueryParams {
    #[page(default)]
    pub page_no: u64,
    #[page(default = 10)]
    pub page_size: u64,
    pub keyword: Option<String>,
}
```

**优先级**：P2（中）

### 12.2 `success!` 宏与 `response` 模块耦合过紧

**问题描述**：`success!` 宏直接操作 `Salvo Response`（写入 body），与 Salvo 框架耦合。未来切换框架时需重写所有 handler。

**优化建议**：

```rust
// 返回 ApiResponse 类型，由框架层自动序列化
pub async fn query_users() -> Json<ApiResponse<Vec<Model>>> {
    let users = service::query_users().await?;
    Json(ApiResponse::success(users))
}
```

**优先级**：P1（中高）

### 12.3 缺少统一的 API 文档注释

**问题描述**：部分端点缺少 `#[oai(...)]` 文档注释，或描述不够详细。Swagger UI 中文档不完整。

**优化建议**：

```rust
#[oai(
    operation_id = "createUser",
    summary = "创建用户",
    description = "创建一个新的系统用户，支持设置姓名、性别、账号、密码等信息"
)]
pub async fn create(
    req: &mut Request, res: &mut Response
) -> Result<(), ApiError> {
    // ...
}
```

**优先级**：P2（中）

### 12.4 空文件需要清理

**问题描述**：以下文件为空，占用代码导航空间：

- `crates/libs/commons/daoyi-cloud-common/src/extract/valid.rs`
- `crates/libs/commons/daoyi-cloud-common/src/openapi/mod.rs`
- `docs/db/demo/ddl.sql`

**优化建议**：删除空文件，在 `mod.rs` 中使用约定注释说明将来会添加的内容，或直接移除 `mod` 声明。

**优先级**：P3（低）

### 12.5 Rust Edition 2024 兼容性风险

**问题描述**：`edition = "2024"` 在 Rust 1.94.0 中是非常新的功能。ECMA 兼容性问题可能导致某些 crate 编译失败。

**优化建议**：评估稳定版依赖是否全部支持 `edition = "2024"`，必要时降级为 `edition = "2021"`。

**优先级**：P2（中）

---

## 13. 依赖管理

### 13.1 部分依赖版本过老或过新

**问题描述**：

| 依赖      | 当前版本        | 最新稳定版  | 风险                    |
|---------|-------------|--------|-----------------------|
| salvo   | 0.93.0      | ~0.97+ | 中                     |
| sea-orm | 2.0.0-rc.38 | ~2.0+  | 中（使用 RC 版，可能 API 不稳定） |
| config  | 0.15.23     | ~0.16+ | 低                     |
| tokio   | 1.52.3      | ~1.52+ | 低（版本合理）               |
| bcrypt  | 0.19.1      | ~0.19+ | 低                     |

**优化建议**：

```toml
# 使用语义版本控制：建议 ^ 前缀
tokio = { version = "1", features = [...] }  # 自动使用 1.x 最新版

# 定期使用 cargo outdated 检查更新
# 安装：cargo install cargo-outdated
# 运行：cargo outdated
```

**优先级**：P2（中）

### 13.2 可选功能依赖过多

**问题描述**：`sea-orm` 引入了 `with-json`、`with-chrono`、`sqlx-mysql`、`runtime-tokio-rustls` 等多个 feature。这些 feature
大部分是必需的，但也有部分可能未使用。

**优化建议**：使用 `cargo udeps`（Unused dependencies）扫描未使用的依赖和功能：

```bash
cargo install cargo-udeps
cargo +nightly udeps  # udeps 需要 nightly
```

**优先级**：P3（低）

### 13.3 缺少 cargo deny 配置

**问题描述**：未使用 `cargo deny` 审计依赖的安全性合规性。

**优化建议**：

```bash
cargo install cargo-deny
cargo deny init
# .deny.toml 包含：
#   - 许可证黑名单/白名单
#   - 已知漏洞检测 (RUSTSEC Advisory DB)
#   - 重复依赖检测
```

**优先级**：P2（中）

---

## 总结：按优先级排列的行动清单

| 优先级    | 编号   | 项目                                              | 预期工作量   | 状态               |
|--------|------|-------------------------------------------------|---------|------------------|
| **P0** | 1.1  | 消除全局单例，改用 AppContext 依赖注入                       | 3-5 天   | ❌ 未开始            |
| **P0** | 3.1  | 引入数据库迁移工具                                       | 0.5-1 天 | ✅ 已完成            |
| **P0** | 5.3  | JWT Secret 默认值移除/生产环境必填                         | 0.5 天   | ✅ 已完成            |
| **P0** | 7.1  | 添加 Prometheus Metrics                           | 1-2 天   | ❌ 未开始            |
| **P0** | 7.4  | 添加健康检查端点                                        | 0.5 天   | ✅ 已完成            |
| **P0** | 8.1  | Service 层单元测试覆盖                                 | 3-5 天   | ❌ 未开始            |
| **P0** | 8.2  | 移除全局单例后的 Mock 测试能力                              | 同 1.1   | ❌ 未开始            |
| **P0** | 11.1 | 添加 Docker 构建配置                                  | 0.5 天   | ✅ 已完成            |
| **P0** | 11.2 | 配置 CI Pipeline                                  | 1 天     | ✅ 已完成            |
| **P1** | 1.2  | 分离领域层（Domain/Infrastructure/API）                | 5-7 天   | ❌ 未开始            |
| **P1** | 2.1  | 配置文件路径改为可配置                                     | 0.5 天   | ✅ 已完成            |
| **P1** | 2.2  | 数据库密码安全存储（环境变量/Vault）                           | 0.5 天   | ❌ 未开始            |
| **P1** | 3.3  | 连接池参数可配置                                        | 0.5 天   | ❌ 未开始            |
| **P1** | 3.4  | 连接健康检查                                          | 0.5 天   | ✅ 已包含于 7.4       |
| **P1** | 4.1  | 引入业务错误码系统                                       | 1-2 天   | ✅ 已完成            |
| **P1** | 4.2  | 所有错误自动记录日志                                      | 0.5 天   | ✅ 已完成            |
| **P1** | 4.3  | RequestId 追踪                                    | 1 天     | ✅ 已完成            |
| **P1** | 5.1  | RBAC 权限模型                                       | 3-5 天   | ❌ 未开始            |
| **P1** | 5.2  | Refresh Token 机制                                | 2-3 天   | ❌ 未开始            |
| **P1** | 5.4  | Token 黑名单/主动失效                                  | 1-2 天   | ❌ 未开始            |
| **P1** | 6.2  | 请求限流中间件                                         | 1 天     | ❌ 未开始            |
| **P1** | 6.3  | API 响应加入 request_id                             | 0.5 天   | ✅ 已完成            |
| **P1** | 6.5  | 全局请求超时                                          | 0.5 天   | ✅ 已完成            |
| **P1** | 7.2  | OpenTelemetry 分布式追踪                             | 2-3 天   | ❌ 未开始            |
| **P1** | 9.1  | HTTPS 支持                                        | 1 天     | ❌ 未开始            |
| **P1** | 9.2  | 密码强度策略升级                                        | 0.5 天   | ✅ 已完成            |
| **P1** | 9.4  | CORS 可配置                                        | 0.5 天   | ❌ 未开始            |
| **P1** | 11.3 | K8s 部署清单                                        | 1-2 天   | ❌ 未开始            |
| **P1** | 12.2 | `success!` 宏 → `json_ok!` + `write_to_response` | 1 天     | ✅ 已完成            |
| **P2** | 1.3  | 微服务边界定义与 gRPC 接口设计                              | 3-5 天   | ❌ 未开始            |
| **P2** | 1.4  | 熔断与重试机制                                         | 2-3 天   | ❌ 未开始            |
| **P2** | 2.4  | 配置 Profile 支持                                   | 1 天     | ✅ 已完成            |
| **P2** | 3.2  | 数据库读写分离                                         | 2-3 天   | ❌ 未开始            |
| **P2** | 3.5  | 慢查询日志                                           | 1 天     | ❌ 未开始            |
| **P2** | 3.6  | 种子数据与 Schema 分离                                 | 0.5 天   | ✅ 已完成（迁移不包含种子数据） |
| **P2** | 5.5  | Refresh Token Rotation                          | 1 天     | ❌ 未开始            |
| **P2** | 6.1  | API 版本管理                                        | 1-2 天   | ❌ 未开始            |
| **P2** | 7.3  | JSON 结构化日志                                      | 0.5 天   | ❌ 未开始            |
| **P2** | 8.3  | Testcontainers 集成测试                             | 2-3 天   | ❌ 未开始            |
| **P2** | 9.3  | HTTP 安全头                                        | 0.5 天   | ❌ 未开始            |
| **P2** | 10.1 | 热点数据缓存                                          | 1-2 天   | ❌ 未开始            |
| **P2** | 10.2 | 数据库连接池预热                                        | 0.5 天   | ❌ 未开始            |
| **P2** | 10.3 | N+1 查询检测与优化                                     | 1 天     | ❌ 未开始            |
| **P2** | 11.4 | Changelog 与版本发布脚本                               | 0.5 天   | ❌ 未开始            |
| **P2** | 12.1 | 宏替代与可维护性改进                                      | 2-3 天   | ❌ 未开始            |
| **P2** | 12.3 | 补全所有 OpenAPI 文档注解                               | 1 天     | ❌ 未开始            |
| **P2** | 12.5 | Rust Edition 兼容性评估                              | 0.5 天   | ❌ 未开始            |
| **P2** | 13.1 | 依赖版本统一与定期更新                                     | 1 天     | ❌ 未开始            |
| **P2** | 13.3 | cargo deny 安全审计                                 | 0.5 天   | ✅ 已完成            |
| **P3** | 6.4  | WebSocket/SSE 支持                                | 2-3 天   | ❌ 未开始            |
| **P3** | 10.4 | 序列化性能优化                                         | 1 天     | ❌ 未开始            |
| **P3** | 12.4 | 清理空文件                                           | 0.25 天  | ✅ 已完成            |
| **P3** | 13.2 | unused features 清理                              | 0.5 天   | ❌ 未开始            |

---

## 实施进度跟踪

> 以下记录每次优化实施的变更内容、时间戳和验证状态。

### 第 1 轮优化（2026-05-16）

**本次完成的项目**：

- 11.2 CI Pipeline `.github/workflows/ci.yml` ✅
- 11.1 Docker 构建配置 `Dockerfile` + `.dockerignore` ✅
- 5.3 JWT Secret 默认值移除 ✅
- 7.4 健康检查端点 `/health` ✅
- 6.5 全局请求超时 (30s) ✅
- 4.3 RequestId 追踪中间件 ✅
- 6.3 API 响应 `request_id` 字段 ✅
- 2.1 配置文件路径 `APP_CONFIG_PATH` 支持 ✅
- 2.4 配置 Profile 支持 ✅
- 12.4 清理空文件 (3 个) ✅
- 13.3 cargo deny 审计配置 ✅

### 第 2 轮优化（2026-05-16）

**本次完成的项目**：

| 编号       | 项目             | 变更文件                                                                                 | 验证状态                        |
|----------|----------------|--------------------------------------------------------------------------------------|-----------------------------|
| **3.1**  | 引入数据库迁移工具      | `crates/migration/` 新建（SeaORM Migration 初始建表）<br>`Cargo.toml` workspace member + dep | ✅ `cargo check --workspace` |
| **4.1**  | 业务错误码系统        | `error/mod.rs` 新增 `ErrorCode` 枚举、`Forbidden` 变体                                      | ✅ 编译通过                      |
| **4.2**  | 错误自动记录日志       | `error/mod.rs` `ApiError::log()` 按级别自动追踪                                             | ✅ 编译通过                      |
| **4.3**  | JWT 中间件简化      | `auth/jwt/middleware.rs` 使用 `write_to_response`                                      | ✅ 编译通过                      |
| **9.2**  | 密码强度升级         | 密码 6→8 位；账号 16→32 位                                                                  | ✅ 编译通过                      |
| **12.2** | `success!` 宏重构 | `response/mod.rs` 废弃旧宏，所有 handler 改用 `json_ok!` + `write_to_response`                | ✅ 编译通过                      |

**重要架构变更** — Handler 模式演进：

```
第 1 代（原始）：success!(res, data) + write_error_response(res, err)
第 2 代（本轮）：json_ok!(res, data) + err.write_to_response(res)
第 3 代（未来）：Result<Json<ApiResponse<T>>, ApiError>  (需 Salvo 0.94+ 支持)
```

第 2 代相比第 1 代：

- 消除对 `success!` 宏的直接依赖
- 错误自动通过 `ApiError::log()` 按级别记录日志
- 统一 `json_ok!` + `write_to_response` 双接口

**变更文件清单（本轮）**：

| 操作 | 文件                                                                  |
|----|---------------------------------------------------------------------|
| 新建 | `crates/migration/Cargo.toml`                                       |
| 新建 | `crates/migration/src/lib.rs`                                       |
| 新建 | `crates/migration/src/m20260516_000001_create_sys_user.rs`          |
| 新建 | `crates/migration/src/bin/main.rs`                                  |
| 重写 | `crates/libs/commons/daoyi-cloud-common/src/error/mod.rs`           |
| 重写 | `crates/libs/commons/daoyi-cloud-common/src/response/mod.rs`        |
| 重写 | `crates/libs/commons/daoyi-cloud-common/src/server/mod.rs`          |
| 重写 | `crates/libs/commons/daoyi-cloud-common/src/auth/jwt/middleware.rs` |
| 重写 | `crates/bins/daoyi-module-demo/src/demo/admin_api/auth/mod.rs`      |
| 重写 | `crates/bins/daoyi-module-demo/src/demo/admin_api/user/mod.rs`      |
| 修改 | `Cargo.toml`（migration member + sea-orm-migration dep）              |
| 修改 | `models/auth.rs`（密码强度提升）                                            |
| 修改 | `models/sys_user.rs`（密码强度 + 字段长度提升）                                 |

**剩余 P0 待实施**：

| 编号  | 项目                 | 原因/前置条件                |
|-----|--------------------|------------------------|
| 1.1 | 全局单例 → DI          | 架构级重构，影响面大，需独立规划       |
| 7.1 | Prometheus Metrics | 需添加 crate 依赖和中间件       |
| 8.1 | Service 层单元测试      | 依赖 1.1（全局单例移除后方可 mock） |
| 8.2 | Mock 测试能力          | 同 1.1                  |

---

## 结论

daoyi-cloud-rs 当前是一个**代码结构清晰、分层合理的单体脚手架项目**，适合快速启动新模块。但要成为**生产可用的 Rust 微服务平台
**，还需重点解决以下三大瓶颈：

1. **架构弹性**（P0）：全局单例 → 依赖注入，这是所有测试能力的基础
2. **可观测性**（P0）：Metrics + Health Check，这是运维能力的基础
3. **基础设施**（P0）：Docker + CI + DB Migration，这是交付能力的基础

建议按 P0 → P1 → P2 → P3 的顺序逐步落地，P0 合计预估 10-15 人天。

---

*本优化指南基于 Rust 2026 年生态系统现状、生产环境运维经验和社区最佳实践编写。*
