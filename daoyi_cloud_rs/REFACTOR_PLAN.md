# Rust 重构计划与任务清单

## 背景与目标
- 以 Rust 2024 + Salvo 0.85.0 + redis 1.0.0-rc.4 + nacos-sdk 0.5.3 + sea-orm 2.0.0-rc.19 重构现有 Java 代码，目录与模块划分与 Maven 版本对齐。
- 按模块（framework/gateway/server/module-*）拆分为 Cargo workspace，支持逐步迁移和并行开发。
- 先完成基础设施（配置、日志、注册中心、连接池），再迁移业务域逻辑，确保可持续交付。

## 目录映射（Rust workspace）
- `daoyi-framework`：基础设施层，承载配置、日志、客户端封装（Redis/Nacos/SeaORM）等。
- `daoyi-gateway`：边界网关，负责转发/鉴权/灰度，依赖基础设施与核心模块。
- `daoyi-server`：聚合服务入口，组合业务模块并提供服务编排。
- `daoyi-module-*`：按业务域拆分的功能模块，与 Java 模块一一对应（system/infra/member/bpm/pay/report/mp/mall/crm/erp/ai/iot）。
- `daoyi-dependencies`：跨模块共享的元数据与未来的公共常量/类型。

## 近期里程碑
1) 基础设施落地：配置模型、日志/链路追踪、Nacos 服务发现与配置拉取、Redis 连接管理、SeaORM 数据源抽象。  
2) 网关/服务骨架：Salvo 路由、健康检查、基础中间件（tracing、request-id、统一响应）。  
3) 域模块支撑：为 system/infra/member 等核心域建立领域模型、服务接口与仓储抽象。  
4) 数据与集成：迁移核心表的 SeaORM Entity/Model/ActiveModel，验证 CRUD + 事务。  
5) 安全与配置：鉴权/鉴权、配置分环境（local/dev/prod），运行/容灾脚本。  
6) 验证与发布：单元/集成测试，容器化/运行脚本，对齐 Java 行为后替换流量。

## 任务清单（迭代优化）
- [ ] Workspace 依赖约定：统一 tokio 运行时、序列化、日志/tracing、错误模型。
- [ ] 配置中心接入：定义 Nacos 客户端封装，支持动态刷新与降级（本地配置）。
- [ ] Redis 连接池：封装连接/序列化工具，提供健壮的健康检查与重试策略。
- [ ] SeaORM 数据访问：建立基础数据库上下文、迁移/实体生成策略，与事务边界约定。
- [ ] HTTP 服务基线：网关/服务的中间件（鉴权、幂等、全局错误处理、CORS、限流）。
- [ ] 模块接口对齐：为 system/infra/member 等模块设计 Service/Repo/DTO 层级，与 Java API 对齐。
- [ ] 领域事件与集成：消息/MQ 协议占位，定义发布/订阅接口，便于后续实现。
- [ ] 测试策略：核心模块的单测/集成测试基线，CI 入口与本地运行脚本。
- [ ] 运维脚本：启动/调试脚本（本地与容器），覆盖配置示例与环境变量。
