# daoyi-entity-demo

Demo 业务的数据层库，包含实体定义、数据模型和服务逻辑。

## 目录结构

```
src/demo/
├── entity/              # SeaORM 实体（数据库表映射）
│   ├── mod.rs
│   ├── prelude.rs
│   └── sys_user.rs      # sys_user 表
├── models/              # 请求/响应模型（含 validator 校验）
│   ├── mod.rs
│   ├── auth.rs          # LoginParams / LoginResult
│   └── sys_user.rs      # UserQueryParams / UserParams
└── service/             # 业务服务（DI 模式，接收 db 参数）
    ├── mod.rs
    ├── auth_service.rs       # 登录 + 单元测试（MockDatabase）
    └── sys_user_service.rs   # 用户 CRUD + 分页
```

## 核心设计

### 依赖注入

所有 Service 函数接收 `db: &DatabaseConnection` 参数，不再使用全局 `db::get()`：

```rust
// auth_service.rs
pub async fn login(db: &DatabaseConnection, params: LoginParams) -> ApiResult<LoginResult>
```

便于单元测试注入 mock DB：

```rust
#[tokio::test]
async fn test_login_disabled_user() {
    let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::MySql)
        .append_query_results([vec![disabled_user()]])
        .into_connection();
    let result = login(&db, params).await;
    assert!(result.is_err());
}
```

### 模型校验规则

| 模型            | 字段           | 规则             |
|---------------|--------------|----------------|
| `LoginParams` | account      | 1-32 位         |
|               | password     | 8-64 位         |
| `UserParams`  | name         | 1-32 位         |
|               | account      | 1-32 位         |
|               | password     | 8-64 位         |
|               | mobile_phone | E.164 格式（正则校验） |

### Entity 生成

```shell
cd crates/libs/entities/daoyi-entity-demo
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity
```

## 依赖关系

```
daoyi-entity-demo
  ├── daoyi-cloud-common  (枚举、分页、校验、JWT)
  ├── sea-orm + serde     (ORM + 序列化)
  ├── validator           (参数校验)
  └── salvo-oapi          (OpenAPI Schema)
```
