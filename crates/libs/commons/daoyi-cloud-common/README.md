# daoyi-cloud-common

daoyi-cloud-rs 项目的公共基础库，封装了 Web 服务开发中常用的基础设施能力，供所有业务模块复用。

## 模块概览

| 模块          | 说明                                           |
|-------------|----------------------------------------------|
| `app`       | 应用启动器，串联配置加载、日志初始化、数据库连接、HTTP 服务器启动          |
| `auth`      | JWT 认证（编解码 + 中间件）                            |
| `conf`      | 全局配置管理（YAML + 环境变量覆盖，OnceCell 单例）            |
| `constants` | 常量定义（默认值、全局值、枚举）                             |
| `db`        | 数据库连接池管理（SeaORM + MySQL）                     |
| `error`     | 统一错误类型 `ApiError`，自动映射 HTTP 状态码              |
| `logger`    | 日志初始化（tracing + chrono 时间格式）                 |
| `pojo`      | 通用 POJO（分页参数 PageParam / 分页结果 PageResult）    |
| `response`  | 统一 API 响应结构 ApiResponse + `success!` 宏       |
| `server`    | HTTP 服务器（Salvo + 中间件栈 + Swagger UI + Scalar） |
| `utils`     | 工具类（ID 生成、密码加密、序列化）                          |

## 核心特性

### 统一错误处理

`ApiError` 枚举覆盖了 Web 开发中常见的错误场景，自动映射为合适的 HTTP 状态码：

| 变体                   | HTTP 状态码 |
|----------------------|----------|
| `NotFound`           | 404      |
| `MethodNotAllowed`   | 405      |
| `Unauthenticated`    | 401      |
| `Validation`         | 422      |
| `Biz`                | 400      |
| `DbErr` / `Internal` | 500      |

### JWT 认证

- HS256 算法签发/验证 Token
- 中间件自动从 `Authorization: Bearer <token>` 提取并校验
- 支持通过配置忽略指定 URL（Glob 模式匹配）
- 校验成功后将 `Principal`（含 `id: i64`, `tenant_id: i64`, `name: String`）注入请求扩展

### 统一响应

所有接口返回 `ApiResponse<T>` 结构：

```json
{
  "code": 0,
  "msg": "",
  "data": {}
}
```

配合 `success!` 宏简化使用：

```rust
success!(result)   // ApiResult<ApiResponse<T>>
success!()         // 无数据返回
```

### 分页

`PageParam` 支持页码/每页条数校验，`PageResult<T>` 泛型分页结果：

```rust
let page = PageResult::from_pagination(params.pagination, total, list);
```

### API 文档

自动集成 Swagger UI 和 Scalar 两套 API 文档界面，共享同一份 OpenAPI 规范。

已内置 OpenAPI 安全方案定义：

| 安全方案        | 类型           | 请求头           | 说明       |
|-------------|--------------|---------------|----------|
| bearer_auth | Http(Bearer) | Authorization | JWT 认证令牌 |
| tenant_id   | ApiKey       | tenant-id     | 租户ID     |

## 依赖关系

```
daoyi-cloud-common
  ├── salvo + salvo-oapi (Web 框架 + OpenAPI 文档)
  ├── sea-orm (ORM)
  ├── jsonwebtoken (JWT)
  ├── bcrypt (密码加密)
  ├── validator (参数校验)
  ├── tracing + tracing-subscriber (日志)
  ├── config (配置管理)
  └── idgenerator + xid (ID 生成)
```
