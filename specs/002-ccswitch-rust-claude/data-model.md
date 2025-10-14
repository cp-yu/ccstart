# Data Model: ccstart 配置管理

**Date**: 2025-10-14
**Branch**: 002-ccswitch-rust-claude

本文档定义 ccstart 项目的核心数据实体和关系模型。

---

## 实体关系图

```
ConfigCollection (config.json)
    ├── ClaudeConfig
    │   └── Providers (HashMap)
    │       ├── Provider 1
    │       │   ├── id: String
    │       │   ├── name: String
    │       │   └── settingsConfig: Value
    │       ├── Provider 2
    │       └── Provider N
    │
    └── [分离后] → SeparatedConfigs (separated/)
            ├── config-<name>.json (Provider 1 的 settingsConfig)
            ├── config-<name>.json (Provider 2 的 settingsConfig)
            └── config-<name>.json (Provider N 的 settingsConfig)
```

---

## 核心实体

### 1. ConfigCollection（配置集合）

**描述**：存储在 `~/.cc-switch/config.json` 中的混合配置数据，包含所有 Claude CLI 的 provider 配置。

**文件路径**：`~/.cc-switch/config.json`

**JSON 结构**：
```json
{
  "claude": {
    "providers": {
      "provider-id-1": {
        "id": "provider-id-1",
        "name": "packycode",
        "settingsConfig": {
          "provider": "anthropic",
          "apiKey": "sk-...",
          "model": "claude-3-5-sonnet-20241022"
        }
      },
      "provider-id-2": {
        "id": "provider-id-2",
        "name": "Zhipu GLM",
        "settingsConfig": {
          "provider": "zhipu",
          "apiKey": "...",
          "baseUrl": "https://open.bigmodel.cn/api/paas/v4/"
        }
      }
    }
  }
}
```

**Rust 类型定义**：
```rust
#[derive(Debug, Deserialize)]
struct ConfigCollection {
    claude: ClaudeConfig,
}

#[derive(Debug, Deserialize)]
struct ClaudeConfig {
    providers: HashMap<String, Provider>,
}
```

**约束**：
- 必须包含 `claude.providers` 字段
- `providers` 至少包含一个 provider
- 符合 JSON 标准格式

---

### 2. Provider（Provider 配置）

**描述**：`config.json` 中 `claude.providers` 下的每个对象，包含 id、name 和 settingsConfig 字段。

**字段定义**：

| 字段 | 类型 | 必需 | 描述 | 示例 |
|------|------|------|------|------|
| `id` | String | 是 | Provider 唯一标识符 | `"provider-123"` |
| `name` | String | 否 | 配置显示名称（用于文件名和命令行） | `"packycode"`, `"Zhipu GLM"` |
| `settingsConfig` | JSON Value | 是 | Claude CLI settings 配置对象 | `{"provider": "anthropic", ...}` |

**Rust 类型定义**：
```rust
#[derive(Debug, Deserialize)]
struct Provider {
    #[serde(default)]
    id: String,

    #[serde(default)]
    name: String,

    #[serde(rename = "settingsConfig")]
    settings_config: serde_json::Value,
}
```

**约束**：
- 如果 `name` 为空，使用 `id` 作为配置名称
- `settingsConfig` 必须是有效的 JSON 对象
- 同一配置集合中的 `name` 可能重复（需要处理）

---

### 3. SeparatedConfig（独立配置）

**描述**：从 Provider 的 `settingsConfig` 中提取的单个配置对象，存储为独立的 JSON 文件。

**文件路径格式**：`~/.cc-switch/separated/config-<encoded-name>.json`

**文件名编码规则**：
- 保留空格、字母、数字、连字符、下划线
- URL 编码特殊字符（`/`, `:`, `*`, `?`, `"`, `<`, `>`, `|`, `\`）
- 示例：
  - `"packycode"` → `config-packycode.json`
  - `"my config"` → `config-my config.json`
  - `"my/config"` → `config-my%2Fconfig.json`

**JSON 结构**（与 Claude CLI settings 一致）：
```json
{
  "provider": "anthropic",
  "apiKey": "sk-...",
  "model": "claude-3-5-sonnet-20241022",
  "baseUrl": "https://api.anthropic.com",
  "otherSettings": "..."
}
```

**Rust 类型定义**：
```rust
// 使用 serde_json::Value 保持原始 JSON 结构
type SeparatedConfig = serde_json::Value;
```

**约束**：
- 文件名必须符合文件系统命名规则
- 文件内容必须是有效的 JSON
- 文件总长度不超过 255 字符（编码后）

---

### 4. ConfigName（配置名称）

**描述**：用于标识和选择特定配置的字符串，来自 Provider 的 `name` 字段。

**来源优先级**：
1. Provider 的 `name` 字段（非空时）
2. Provider 的 `id` 字段（`name` 为空时）

**重复处理策略**：
- 首次出现：保持原名称（如 `"default"`）
- 重复出现：添加数字后缀（如 `"default-2"`, `"default-3"`）

**使用场景**：
- 命令行参数：`ccstart <config-name>`
- 文件名：`config-<encoded-config-name>.json`
- Shell 补全候选项

**约束**：
- 不能为空字符串
- 编码后的文件名长度不超过 255 字符
- 包含空格或特殊字符时，shell 中需要双引号包裹

---

## 数据流

### 初始化流程（`ccstart init`）

```
1. 读取 config.json
   ↓
2. 解析为 ConfigCollection
   ↓
3. 遍历 claude.providers
   ↓
4. 对每个 Provider：
   - 提取 name（或使用 id）
   - 检查重复并添加后缀
   - 编码配置名称
   - 生成文件名：config-<encoded-name>.json
   ↓
5. 将 settingsConfig 写入 separated/ 目录
   ↓
6. 完成：生成 N 个 SeparatedConfig 文件
```

### 更新流程（`ccstart update`）

```
1. 读取 config.json
   ↓
2. 读取 separated/ 目录现有文件
   ↓
3. 对比差异：
   - 新增的 provider → 创建新文件
   - 已存在的 provider → 更新文件内容
   - 删除的 provider → 删除对应文件
   ↓
4. 同步完成
```

### 配置切换流程（`ccstart <name>`）

```
1. 接收配置名称参数
   ↓
2. 编码配置名称
   ↓
3. 查找文件：~/.cc-switch/separated/config-<encoded-name>.json
   ↓
4. 如果文件存在：
   - 调用：claude --settings <文件路径> [额外参数]
   - 透传退出码
   ↓
5. 如果文件不存在：
   - 显示错误：配置 '<name>' 不存在
   - 列出可用配置
   - 退出码：1
```

---

## 状态管理

### 配置状态（可选扩展）

**文件**：`~/.cc-switch/.ccstart-state.json`

**用途**：
- 记录上次使用的配置
- 记录初始化时间戳
- 记录配置文件版本

**JSON 结构**（可选）：
```json
{
  "lastUsedConfig": "packycode",
  "lastInitTime": "2025-10-14T10:30:00Z",
  "configVersion": "1",
  "totalConfigs": 5
}
```

**注意**：此功能为可选增强，MVP 不需要实现。

---

## 验证规则

### ConfigCollection 验证

```rust
fn validate_config_collection(config: &ConfigCollection) -> Result<()> {
    // 1. 检查 providers 非空
    if config.claude.providers.is_empty() {
        anyhow::bail!("配置文件中没有找到任何 provider");
    }

    // 2. 检查每个 provider
    for (id, provider) in &config.claude.providers {
        // settingsConfig 不能为 null
        if provider.settings_config.is_null() {
            eprintln!("警告: Provider '{}' 的 settingsConfig 为空", id);
        }
    }

    Ok(())
}
```

### ConfigName 验证

```rust
fn validate_config_name(name: &str) -> Result<()> {
    // 1. 不能为空
    if name.is_empty() {
        anyhow::bail!("配置名称不能为空");
    }

    // 2. 编码后的文件名长度检查
    let filename = format!("config-{}.json", encode_config_name(name));
    if filename.len() > 255 {
        anyhow::bail!(
            "配置名称过长，编码后文件名超过 255 字符（当前：{}）",
            filename.len()
        );
    }

    Ok(())
}
```

### SeparatedConfig 文件验证

```rust
fn validate_separated_config(path: &Path) -> Result<()> {
    // 1. 文件存在
    if !path.exists() {
        anyhow::bail!("配置文件不存在: {}", path.display());
    }

    // 2. 文件可读
    let content = fs::read_to_string(path)
        .context("无法读取配置文件")?;

    // 3. JSON 格式有效
    serde_json::from_str::<serde_json::Value>(&content)
        .context("配置文件 JSON 格式错误")?;

    Ok(())
}
```

---

## 错误场景

### 1. 配置文件不存在

**场景**：用户运行 `ccstart init`，但 `~/.cc-switch/config.json` 不存在。

**处理**：
```
错误: 配置文件不存在: ~/.cc-switch/config.json
提示: 请确保已安装 ccswitch 并创建了配置文件。
退出码: 1
```

### 2. JSON 格式错误

**场景**：`config.json` 包含语法错误。

**处理**：
```
错误: JSON 格式错误位于第 10 行第 5 列
提示: 检查该位置附近是否有多余的逗号、缺少引号或括号不匹配
退出码: 1
```

### 3. 配置名称重复

**场景**：多个 provider 具有相同的 `name` 字段。

**处理**：
```
警告: 检测到重复的配置名称 'default'
  - 第 1 个保持原名: default
  - 第 2 个添加后缀: default-2
  - 第 3 个添加后缀: default-3
```

### 4. 配置不存在

**场景**：用户运行 `ccstart nonexistent`。

**处理**：
```
错误: 配置 'nonexistent' 不存在

可用配置:
  - packycode
  - "Zhipu GLM"
  - work
  - default

提示: 使用 'ccstart list' 查看所有配置
退出码: 1
```

---

## 性能考虑

### 配置文件大小

**假设**：
- 单个配置文件 < 10MB（spec.md 约束）
- 配置数量 < 50 个（spec.md 测试目标）

**性能指标**：
- 解析单个配置文件：< 100ms
- 分离所有配置：< 5秒（spec.md 要求）
- 切换配置启动 Claude：< 1秒（spec.md 要求）

### 内存占用

**估算**：
- ConfigCollection 解析：< 50MB（50 个配置 × 1MB 平均大小）
- SeparatedConfig 写入：流式写入，内存占用可忽略
- 总内存使用：< 100MB

---

## 扩展性

### 未来可能的扩展

1. **配置版本控制**：
   - 在 `.ccstart-state.json` 中记录配置版本
   - 支持配置迁移和回滚

2. **配置验证**：
   - 验证 settingsConfig 的 Claude CLI schema
   - 提前检测配置错误

3. **配置导入/导出**：
   - 导出单个配置到独立文件
   - 从独立文件导入到 config.json

4. **配置模板**：
   - 提供常见 provider 的配置模板
   - 快速创建新配置

5. **配置搜索**：
   - 支持模糊搜索配置名称
   - 支持按 provider 类型过滤

---

## 参考

- **Claude CLI Settings Format**: https://docs.anthropic.com/claude/cli
- **spec.md**: 功能规格定义
- **research.md**: 技术选型调研
