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