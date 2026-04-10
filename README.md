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

# 升级依赖

```shell
# 安装cargo-edit（包含upgrade）
cargo install cargo-edit

# 升级所有依赖到最新兼容版本（同时更新 Cargo.toml）
cargo upgrade

# 只查看会更新哪些内容，不实际执行（推荐先跑这个）
cargo upgrade --dry-run

# 指定某个包升级
cargo upgrade -p axum

# 升级到最新版本（允许 breaking change）
cargo upgrade --upgrade all

# 升级类型选择
cargo upgrade --upgrade patch     # 只升补丁版本 (0.8.1 → 0.8.2)
cargo upgrade --upgrade minor     # 升小版本 (0.8.x → 0.9.x)
cargo upgrade --upgrade all       # 全部升级 (0.8.x → 1.x)
cargo upgrade --upgrade compatible # 默认行为，兼容性升级

# 升级到 Cargo.lock 中记录的版本号
cargo upgrade --to-lockfile

```

# 推送到github

```config
[core]
	repositoryformatversion = 0
	filemode = true
	bare = false
	logallrefupdates = true
[remote "origin"]
	url = git@gitee.com:daoyi2026/daoyi-cloud-rs.git
	pushurl = git@gitee.com:daoyi2026/daoyi-cloud-rs.git
	pushurl = git@github.com:gemiman/daoyi-cloud-rs.git
	fetch = +refs/heads/*:refs/remotes/origin/*
[branch "master"]
	remote = origin
	merge = refs/heads/master
[remote "github"]
	url = git@github.com:gemiman/daoyi-cloud-rs.git
	fetch = +refs/heads/*:refs/remotes/github/*

```
```shell
# 推送到两个平台
git push origin master

# 或者分开推送
git push origin master   # Gitee + GitHub
git push github master   # 仅 GitHub

```