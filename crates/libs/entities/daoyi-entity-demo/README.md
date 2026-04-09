# 生成entity

```shell
sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/demo/entity
```