# daoyi-module-demo

Demo 业务的 API 路由层，定义 HTTP 接口并处理请求，支持独立部署为微服务。

## 两种运行模式

```shell
# 独立模式（端口 28080）
RUST_LOG=info RUST_LOG_FORMAT=json cargo run -p daoyi-module-demo

# 聚合模式（通过主应用，端口 38080）
RUST_LOG=info RUST_LOG_FORMAT=json cargo run
```

## API 接口

所有接口前缀 `/admin-api/demo`，除登录外均需 JWT Bearer Token。

### 认证管理

| 方法   | 路径                               | 说明                     | 认证 |
|------|----------------------------------|------------------------|----|
| POST | `/admin-api/demo/auth/login`     | 用户登录（返回 JWT + RBAC 权限） | 无  |
| GET  | `/admin-api/demo/auth/user-info` | 获取当前用户信息               | 需要 |

### 用户管理

| 方法     | 路径                           | 说明      | 认证 |
|--------|------------------------------|---------|----|
| GET    | `/admin-api/demo/users`      | 查询所有用户  | 需要 |
| GET    | `/admin-api/demo/users/page` | 分页查询    | 需要 |
| POST   | `/admin-api/demo/users`      | 创建用户    | 需要 |
| PUT    | `/admin-api/demo/users/{id}` | 更新用户    | 需要 |
| GET    | `/admin-api/demo/users/{id}` | 按 ID 查询 | 需要 |
| DELETE | `/admin-api/demo/users/{id}` | 删除用户    | 需要 |

## 目录结构

```
src/
├── main.rs                   # 独立启动入口
├── lib.rs                    # 库入口
└── demo/
    ├── mod.rs                # 路由定义（JWT + 安全方案 + 子路由）
    └── admin_api/
        ├── mod.rs            # 路由聚合
        ├── auth/mod.rs       # 认证接口（DI 模式：从 Depot 获取 AppContext）
        └── user/mod.rs       # 用户 CRUD 接口
```

## 请求处理流程

```
Request
  → InjectCtx (注入 AppContext)
  → RateLimit (限流)
  → SecurityHeaders (6 个安全头)
  → Metrics (计数)
  → RequestId (追踪)
  → CORS (跨域)
  → Timeout (30s)
  → Router 匹配
    → JwtAuthHandler (Bearer Token 校验，跳过 ignore_urls)
    → Handler 函数 (从 Depot 获取 AppContext)
      → 参数提取 (JSON/Query/Path)
      → Service 调用
      → 统一响应
```

## 依赖关系

```
daoyi-module-demo
  ├── daoyi-cloud-common  (AppContext、提取器、响应、JWT、RBAC)
  ├── daoyi-entity-demo   (实体、模型、服务)
  └── sea-orm             (DatabaseConnection 类型)
```
