# daoyi-module-demo

Demo 业务的 API 路由层，定义 HTTP 接口并处理请求，可独立部署为微服务。

## 两种运行模式

```shell
# 独立模式（端口 28080，加载 application-demo.yaml）
RUST_LOG=DEBUG cargo run -p daoyi-module-demo

# 聚合模式（通过主应用启动，端口 38080，加载 application-server.yaml）
RUST_LOG=DEBUG cargo run
```

## API 接口

所有接口前缀为 `/admin-api/demo`，除登录外均需 JWT 认证（Bearer Token + tenant-id 请求头）。

### 认证管理

| 方法   | 路径                               | 说明       | 认证 |
|------|----------------------------------|----------|----|
| POST | `/admin-api/demo/auth/login`     | 用户登录     | 无  |
| GET  | `/admin-api/demo/auth/user-info` | 获取当前用户信息 | 需要 |

### 用户管理

| 方法     | 路径                           | 说明         | 认证 |
|--------|------------------------------|------------|----|
| GET    | `/admin-api/demo/users`      | 查询所有用户     | 需要 |
| GET    | `/admin-api/demo/users/page` | 分页查询用户     | 需要 |
| POST   | `/admin-api/demo/users`      | 创建用户       | 需要 |
| PUT    | `/admin-api/demo/users/{id}` | 更新用户       | 需要 |
| GET    | `/admin-api/demo/users/{id}` | 根据 ID 查询用户 | 需要 |
| DELETE | `/admin-api/demo/users/{id}` | 删除用户       | 需要 |

## 目录结构

```
src/
├── main.rs                # 独立启动入口
├── lib.rs                 # 库入口
└── demo/
    ├── mod.rs             # Demo 路由定义（JWT 中间件 + 安全方案 + 子路由聚合）
    └── admin_api/
        ├── mod.rs         # Admin API 路由聚合 + OpenAPI 合并
        ├── auth/
        │   └── mod.rs     # 认证接口（登录 / 获取用户信息）
        └── user/
            └── mod.rs     # 用户 CRUD 接口
```

## 路由架构

```
/admin-api                          ← JWT 认证中间件 + OpenAPI 安全方案（bearer_auth + tenant_id）
  /demo
    /auth
      POST /login                   ← 忽略 JWT 校验
      GET  /user-info
    /users
      GET  /
      GET  /page
      POST /
      PUT  /{id}
      GET  /{id}
      DELETE /{id}
```

每个子模块返回 `Router`，通过 `#[endpoint]` 宏声明路由和 OpenAPI 文档信息，父模块合并路由，最终统一注册到 HTTP 服务器。

## 依赖关系

```
daoyi-module-demo
  ├── daoyi-cloud-common    # 公共基础（AppState、提取器、响应、JWT 等）
  ├── daoyi-entity-demo     # 实体、模型、服务
  └── salvo-oapi            # OpenAPI 路径注解
```
