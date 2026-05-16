# daoyi-cloud-common

daoyi-cloud-rs 项目的公共基础设施库，封装 Web 服务开发中常用的基础设施，供所有业务模块复用。

## 模块概览

| 模块          | 说明                                                  |
|-------------|-----------------------------------------------------|
| `app`       | 应用启动器：dotenvy → 配置加载 → 日志 → ID → JWT → DB → HTTP    |
| `auth`      | JWT 认证（HS256 / RBAC / Token 黑名单 / Refresh Token）    |
| `cache`     | 简单内存缓存（线程安全 + TTL 过期）                               |
| `conf`      | 配置管理（YAML + 环境变量 + Profile 覆盖）                      |
| `constants` | 常量定义（默认值、全局值、Gender 枚举）                             |
| `context`   | AppContext 依赖注入（InjectContext 中间件 + from_depot 提取）  |
| `db`        | MySQL 连接池管理（参数可配置）                                  |
| `error`     | ApiError + ErrorCode + IntoResponse + 自动日志          |
| `extract`   | 请求参数提取/校验（JSON / Query / Path）                      |
| `logger`    | 日志初始化（文本 / JSON 双模式，`RUST_LOG_FORMAT` 切换）           |
| `pojo`      | 分页参数 + 分页结果 + `page_query_params!` 宏                |
| `response`  | ApiResponse 统一响应结构 + `json_ok!` 宏                   |
| `server`    | AppServer（CORS / Metrics / 限流 / 超时 / 安全头 / OpenAPI） |
| `utils`     | 雪花 ID / bcrypt 密码 / 计时宏 / 序列化工具                     |

## 核心特性

### 依赖注入

```rust
// AppContext 持有 db + jwt，通过 InjectContext 注入到每个请求
let ctx = AppContext::from_depot(depot).unwrap();
let result = my_service::query(&ctx.db, params).await?;
```

### 统一错误处理

```rust
pub enum ApiError {
    NotFound, MethodNotAllowed, Biz(String),
    Internal(anyhow::Error), DbErr(DbErr),
    Validation(String), Bcrypt(BcryptError),
    JWT(JWTError), Unauthenticated(String),
    Forbidden(String), Glob(BuildError),
}
```

每个变体自动映射 HTTP 状态码 + 业务错误码（ErrorCode 枚举），并自动记录日志。

### 统一响应

```json
{ "code": 0, "msg": "操作成功", "data": {}, "requestId": "xid" }
```

### JWT + RBAC

- HS256 签发/验证
- `Principal` 含 `roles` / `permissions` 字段
- `RequirePermission` / `RequireRole` 中间件
- `TokenBlacklist` 支持主动撤销
- `encode_refresh` / `decode_refresh` 支持 Refresh Token

### 中间件栈

```
InjectCtx → RateLimit → SecurityHeaders → Metrics → RequestId → CORS → Timeout
```

### API 文档

Swagger UI + Scalar，已内置 `bearer_auth` 和 `tenant_id` 安全方案。

## 依赖关系

```
daoyi-cloud-common
  ├── salvo + salvo-oapi (Web 框架 + OpenAPI)
  ├── sea-orm (ORM + mock)
  ├── jsonwebtoken + bcrypt (认证)
  ├── validator (参数校验)
  ├── tracing + tracing-subscriber (日志 + JSON)
  ├── config + dotenvy (配置)
  └── idgenerator + xid (ID 生成)
```
