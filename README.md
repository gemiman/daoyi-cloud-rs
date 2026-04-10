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

# 1️⃣ 预览（推荐第一步）
cargo upgrade --dry-run

# 2️⃣ 兼容性升级（默认行为，等于 --compatible）
cargo upgrade

# 3️⃣ 不兼容升级（跨大版本，如 axum 0.8 → 0.9）
cargo upgrade -i

# 4️⃣ 指定单个包升级
cargo upgrade -p axum
cargo upgrade -p axum@0.9          # 指定目标版本

# 5️⃣ 排除某些包，其余全部升级（含不兼容）
cargo upgrade -i --exclude sea-orm

# 6️⃣ 组合使用：允许不兼容 + 允许固定版本也升
cargo upgrade -i --pinned allow


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