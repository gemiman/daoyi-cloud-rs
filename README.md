# 安装sea-orm-cli

```shell
cargo install sea-orm-cli@^2.0.0-rc
```

# 生成entity

```shell
sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/entity
```

# 启动

```shell
RUST_LOG=DEBUG cargo run
```