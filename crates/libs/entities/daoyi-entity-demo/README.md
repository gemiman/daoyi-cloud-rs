# daoyi-entity-demo

Demo 业务的数据层库，包含实体定义、数据模型和服务逻辑，采用三层架构组织。

## 目录结构

```
src/demo/
├── entity/            # SeaORM 实体（数据库表映射）
│   ├── mod.rs
│   ├── prelude.rs     # 实体别名
│   └── sys_user.rs    # sys_user 表实体
├── models/            # 请求/响应模型
│   ├── mod.rs
│   ├── auth.rs        # 登录参数 LoginParams、登录结果 LoginResult
│   └── sys_user.rs    # 用户查询参数 UserQueryParams、用户操作参数 UserParams
└── service/           # 业务服务层
    ├── mod.rs
    ├── auth_service.rs       # 认证服务（登录逻辑）
    └── sys_user_service.rs   # 用户服务（CRUD + 分页）
```

## 三层架构

### Entity 层

由 `sea-orm-cli` 自动生成，对应数据库表结构，支持 SeaORM 的完整 CRUD 操作。

当前实体：

| 实体         | 表名       | Schema | 说明    |
|------------|----------|--------|-------|
| `sys_user` | sys_user | demo   | 系统用户表 |

### Model 层

定义 API 接口的请求参数和响应模型，集成 `validator` 校验和 `salvo_oapi::ToSchema` 文档生成：

| 模型                | 用途      | 校验规则                                          |
|-------------------|---------|-----------------------------------------------|
| `LoginParams`     | 登录请求    | account: 1-16 位, password: 6-16 位             |
| `LoginResult`     | 登录响应    | access_token                                  |
| `UserQueryParams` | 用户查询    | keyword + PageParam                           |
| `UserParams`      | 用户新增/编辑 | name/account/password 长度限制, mobile_phone 格式校验 |

### Service 层

封装业务逻辑，供上层 API Handler 调用：

| 服务                 | 方法                  | 说明                         |
|--------------------|---------------------|----------------------------|
| `auth_service`     | `login`             | 查账号 → 检查启用 → 验证密码 → 生成 JWT |
| `sys_user_service` | `query_page`        | 关键词 + 分页查询                 |
|                    | `query_users`       | 组合条件查询全部                   |
|                    | `create_user`       | 自动递增 ID + 密码哈希             |
|                    | `update_user_by_id` | 密码为空则不更新                   |
|                    | `get_user_by_id`    | 按 ID 查询                    |
|                    | `delete_user_by_id` | 按 ID 删除                    |

## 生成 Entity

当数据库表结构变更时，重新生成 Entity：

```shell
cd crates/libs/entities/daoyi-entity-demo

sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  -s demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity
```

## 依赖关系

```
daoyi-entity-demo
  ├── daoyi-cloud-common    # 公共基础（枚举、分页、校验等）
  ├── sea-orm               # ORM 核心
  ├── serde                 # 序列化/反序列化
  ├── validator             # 参数校验
  ├── salvo-oapi            # OpenAPI Schema 生成
  └── bcrypt                # 密码哈希（auth_service 使用）
```
