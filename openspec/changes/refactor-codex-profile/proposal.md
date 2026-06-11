<!-- Smart routing: Design Summary found in explore session. Proceeding directly. -->

## Why

当前 codex dispatch 仅提取 4 个字段通过 `-c` 逐一注入，丢失了 `mcp_servers`、`projects`、`reasoning_effort`、`web_search` 等完整配置。Codex ≥0.134.0 提供了 `-p` (profile) 机制，可将完整 TOML 配置文件作为命名层叠加，天然适配 ccstart 的 Read-Through Cache 模式。同时需要支持中文渠道名的拼音首字母模糊匹配和 `cxstart` 快捷入口。

## What Changes

- 替换 `-c` 逐字段注入为 `-p` profile 文件方式启动 codex
- Profile 文件命名采用 `ccstart-<sha256(channel_name)[..8]>` 格式，解决中文名称不兼容 codex profile 命名规则的问题
- 引入拼音首字母匹配：当精确查找失败且输入为纯 ASCII 字母时，按首字母匹配中文渠道名
- 新增 `cxstart` 快捷入口（argv[0] 检测），等价于 `ccstart codex`
- `ccstart update` 对称支持 codex profile 缓存的同步与清理
- Shell 补全支持拼音首字母候选

## Capabilities

### New Capabilities
- `codex-profile-cache`: Codex profile 文件的 Read-Through Cache 管理（生成、哈希比较、原子写入、过期清理）
- `pinyin-resolve`: 拼音首字母模糊匹配渠道名称（精确优先、碰撞检测、补全集成）
- `cxstart-shortcut`: argv[0] 检测实现 `cxstart` 快捷入口，隐含 codex 子命令

### Modified Capabilities
- `codex-dispatch`: 从 `-c` 逐字段注入改为 `-p` profile 文件启动
- `codex-completion`: 补全候选增加拼音首字母匹配

## Impact

- `src/commands/codex.rs` — 重写核心逻辑
- `src/config/` — 新增 codex profile cache 模块
- `src/main.rs` — argv[0] 检测路由、补全函数更新
- `Cargo.toml` — 新增 pinyin crate 依赖
- `.github/workflows/release.yml` — CI 产出 `cxstart` 副本
- `~/.codex/` — 运行时生成 `ccstart-*.config.toml` 文件
