# Changelog

## [0.9.0] - 2026-05-16

### Added

- 项目整体架构与核心功能：Salvo + SeaORM + MySQL 微服务脚手架
- 聚合模式（单体部署）与独立模式（微服务部署）双模式支持
- 统一 API 响应结构 `ApiResponse`、统一错误处理 `ApiError`
- JWT 认证中间件
- 用户 CRUD + 登录认证 Demo 模块
- OpenAPI / Swagger UI / Scalar 文档自动生成

### Optimized (6 轮优化, 2026-05-16)

#### Infrastructure

- CI Pipeline (GitHub Actions: check/fmt/clippy/test/build/audit)
- Docker 多阶段构建 (Dockerfile + .dockerignore)
- K8s 部署清单 (Deployment/Service/HPA/Secret/ConfigMap)
- SeaORM 数据库迁移框架 (crates/migration)
- cargo-deny 依赖审计配置

#### Observability

- Prometheus Metrics 端点 (/metrics: 请求数/活跃数/4xx/5xx)
- 健康检查端点 (/health: Liveness + Readiness)
- RequestId 追踪 (Salvo RequestId 中间件 + ApiResponse.request_id 字段)
- JSON 结构化日志 (RUST_LOG_FORMAT=json 环境变量)
- 全局请求超时 (30s Timeout 中间件)

#### Security

- JWT Secret 默认值移除，启动时强制校验
- 密码强度升级 (6→8 位)
- CORS 可配置 (server.cors YAML 配置节)
- HTTP 安全头中间件 (6 个安全头)
- 请求限流中间件 (固定窗口，1000/min)
- dotenvy .env 文件加载，数据库密码支持环境变量

#### Architecture

- 全局单例 → AppContext 依赖注入 (可测性基础)
- 所有 Service 层 DI 改造 (接收 db 参数)
- SeaORM MockDatabase 单元测试 (2 个 auth 测试)
- 业务错误码系统 (ErrorCode 枚举)
- 错误自动日志 (ApiError::log() 按级别记录)
- success! 宏 → json_ok! + write_to_response 重构
- 配置 Profile 支持 (application-{profile}.yaml)
- 连接池参数可配置 (database.pool YAML 配置节)

#### Code Quality

- 清理 3 个空文件
