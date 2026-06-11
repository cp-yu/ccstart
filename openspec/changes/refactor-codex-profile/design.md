## Context

当前 `ccstart codex` 实现从数据库提取 4 个字段（model_provider, model, base_url, env_key），通过 `-c key=value` 逐字段注入 codex 命令行。这导致数据库中完整的 TOML 配置（mcp_servers, projects, reasoning_effort, web_search 等）丢失。

Codex ≥0.134.0 引入 `-p / --profile` 机制：将 `~/.codex/<name>.config.toml` 整体叠加到基础配置，提供完整的命名配置层。

## Goals / Non-Goals

**Goals:**
- 利用 codex `-p` profile 机制，将数据库 TOML 配置**原样写入** profile 文件
- 实现 Read-Through Cache（与 claude 缓存对称），通过内容哈希判断是否需要更新
- 支持中文渠道名（用户侧），profile 文件名使用 `ccstart-<sha256(name)[..8]>` 映射
- 支持拼音首字母模糊匹配（仅 ASCII 输入时触发）
- 提供 `cxstart` 快捷入口（argv[0] 检测 + CI symlink）
- `ccstart update` 对称管理 codex profile 文件的同步和清理

**Non-Goals:**
- 不支持 codex <0.134.0 的旧 `[profiles.xxx]` 内嵌语法
- 不修改 claude 侧的缓存逻辑
- 不实现全拼匹配（仅首字母）

## Decisions

**Profile 文件命名**: `ccstart-<sha256(channel_name)[..8]>.config.toml`
- 原因：codex `-p` 要求 `[a-zA-Z0-9_-]`，中文名无法直接使用；8 位 hex 在十几个 channel 场景下碰撞概率可忽略

**配置写入策略**: 数据库 `settings_config.config` TOML 原样写入，不做字段过滤
- 原因：每个 channel 是完整独立环境，无需与 base config 做差异计算

**认证注入**: 仍通过环境变量 `OPENAI_API_KEY` 注入（不写入 profile 文件）
- 原因：密钥不应持久化到磁盘文件

**名称解析优先级**:
1. 精确匹配
2. 拼音首字母匹配（仅 input 全为 ASCII 字母时）
   - 唯一命中 → 使用
   - 多个命中 → 列出候选，退出
   - 无命中 → 报错

**argv[0] 检测**: `basename(argv[0]) == "cxstart"` 时隐含 codex 子命令
- CI release 产出 `cxstart` 副本（hardcopy 或 symlink）

**拼音依赖**: 引入轻量 pinyin crate，仅使用首字母提取功能

## Risks / Trade-offs

| 风险 | 应对 |
|------|------|
| pinyin crate 体积增长 | 选择轻量实现，评估编译后增量 |
| 8 位 hash 碰撞 | 在十几个 channel 规模下概率 ~10⁻⁷；碰撞时可加长 |
| `~/.codex/` 下残留过期 profile | `ccstart update` 清理 `ccstart-*` 前缀文件 |
| codex 未来变更 `-p` 语义 | 仅支持 ≥0.134.0，profile 为标准 TOML，耦合度低 |
| 密钥仅在 env 中，profile 文件不含认证 | 如果 codex 不读 env 则需要额外处理；当前 codex 支持 `OPENAI_API_KEY` env |
