# Repository Guidelines

## 项目结构与模块划分
- 后端为 Maven 多模块：`yudao-framework` 提供通用 starter 与基础组件，`yudao-module-*` 按业务域拆分（system/infra/member 等 API + server 子模块），`yudao-server` 作为聚合启动壳；网关在 `yudao-gateway`。
- 前端脚手架位于 `yudao-ui/` 下的各子目录（Vue3/Vben/Vue2/uni-app 方案），当前仓库仅留占位说明。
- 初始化 SQL 在 `sql/`，本地/容器脚本在 `script/`（含 docker-compose）。
- 主要配置：`yudao-server/src/main/resources/application*.yaml`，请用 `application-local.yaml` 覆盖个人敏感配置。

## 构建、测试与本地运行
- 构建全量（跳过测试）：`mvn -T 1C clean package -DskipTests`。
- 仅后端主服务打包：`mvn -pl yudao-server -am clean package -DskipTests`。
- 本地启动：`mvn -pl yudao-server -am spring-boot:run -DskipTests`，需先准备 Nacos/Redis/DB（可用 `script/docker/docker-compose.yml`）。
- 运行单测：`mvn -pl yudao-module-system-server test`（替换为目标模块）；提交前至少确保关联模块测试通过。

## 代码风格与命名
- Java 17 + Spring Boot 3，遵循《阿里巴巴 Java 开发手册》；缩进 4 空格，保持包名全小写，类名/方法名用驼峰。
- 使用 Lombok + MapStruct，注意开启 IDE 注解处理器；DTO/VO/BO 分层命名保持一致。
- 配置常量集中在 `yudao-framework` 对应 starter 中，公共工具优先复用 `yudao-common`。

## 测试约定
- 测试位于各模块的 `src/test/java`，默认 JUnit 5 + Spring Boot Test；集成测试命名 `*Test`/`*IT`。
- 新增业务需至少覆盖核心 Service/Mapper 逻辑；如修改安全、缓存、MQ 相关，请补充回归用例。
- 本地可使用 `@SpringBootTest` + 内存/Mock 依赖，避免依赖真实三方服务。

## 提交与合并请求
- Git 历史采用类似 Conventional Commits，如 `feat(config): ...`、`fix(iot): ...`、`chore(project): ...`；信息需突出变更点与影响范围。
- PR 描述需包含：变更摘要、影响模块、测试结果（命令输出摘要）；涉及界面改动的，请附截图或交互说明。
- 与 Issue 关联时在描述中引用编号（如 `Fixes #123`），保持讨论集中；避免在同一 PR 混入无关重构。

## 配置与安全提示
- 不要提交本地密钥、数据库账户等，敏感信息统一放入本地 `application-local.yaml` 或环境变量。
- 生产配置（如限流、鉴权）在 `yudao-spring-boot-starter-protection` 等 starter 中集中管理，修改需说明风险与回滚方案。
## codex 提示
    ```shell
    To continue this session, run codex resume 019abf68-6fb0-7350-95d6-274239e35860
    ```