[Root Directory](../../../CLAUDE.md) > [src](../../) > **utils**

# Utils 模块文档

> 模块职责：提供文件系统操作和字符串编码功能的基础工具支持

## 导航面包屑
- **父目录**: [src](../../CLAUDE.md)
- **根目录**: [项目根目录](../../../CLAUDE.md)

## 模块职责

Utils 模块为 ccstart 提供基础工具功能，主要包括文件系统操作和配置名称的安全编码处理。这些工具函数被其他模块广泛使用，确保了代码的重用性和一致性。

## 入口和启动

### 模块入口
- **主文件**: `mod.rs` - 模块声明和导出
- **核心组件**:
  - `fs.rs` - 文件系统操作和路径管理
  - `encoding.rs` - 字符串编码和解码

### 设计原则
- **安全优先**: 所有文件操作都包含适当的错误处理
- **跨平台兼容**: 考虑不同操作系统的路径和编码差异
- **原子操作**: 关键文件写入操作确保原子性

## 外部接口

### fs.rs - 文件系统工具

#### 核心结构体
```rust
#[derive(Debug, Clone)]
pub struct ConfigPaths {
    pub base_dir: PathBuf,      // ~/.cc-switch
    pub config_json: PathBuf,   // ~/.cc-switch/config.json
    pub separated_dir: PathBuf, // ~/.cc-switch/separated/
}
```

#### 核心函数
```rust
pub fn get_config_paths() -> AppResult<ConfigPaths>
```
- **功能**: 返回 ccstart 使用的标准路径集合
- **返回**: 包含所有关键路径的 `ConfigPaths` 结构
- **路径规则**: 基于用户主目录的固定路径结构

```rust
pub fn expand_tilde(path: &str) -> AppResult<PathBuf>
```
- **功能**: 展开以 `~` 开头的路径为用户主目录
- **参数**: `path` - 可能包含 `~` 的路径字符串
- **支持**: `~` 和 `~/sub/path` 两种格式
- **错误**: 无法获取用户主目录时返回错误

```rust
pub fn ensure_dir_exists(dir: &Path) -> AppResult<()>
```
- **功能**: 确保目录存在，不存在则创建
- **参数**: `dir` - 目录路径
- **行为**: 递归创建所有必要的父目录
- **原子性**: 使用 `fs::create_dir_all` 确保原子性

### encoding.rs - 编码工具

#### 编码规则
```rust
const FORBIDDEN: &AsciiSet = &CONTROLS
    .add(b'/')
    .add(b':')
    .add(b'*')
    .add(b'?')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'|')
    .add(b'\\');
```

#### 核心函数
```rust
pub fn encode_config_name(name: &str) -> String
```
- **功能**: 将配置名称编码为安全的文件名
- **参数**: `name` - 原始配置名称
- **返回**: URL 百分号编码后的字符串
- **保留字符**: 字母、数字、连字符、下划线、空格
- **编码字符**: 文件系统不安全字符

```rust
pub fn decode_config_name(encoded: &str) -> anyhow::Result<String>
```
- **功能**: 解码配置名称中的百分号编码
- **参数**: `encoded` - 编码后的字符串
- **返回**: 解码后的原始配置名称
- **错误**: UTF-8 解码失败时返回错误

## 关键依赖和配置

### 外部依赖
- `dirs` - 用户目录检测和路径处理
- `percent-encoding` - URL 编码实现
- `anyhow` - 错误处理

### 内部依赖
- `crate::error` - 错误类型定义

### 路径配置
- **基础目录**: `~/.cc-switch/`
- **配置文件**: `~/.cc-switch/config.json`
- **分离目录**: `~/.cc-switch/separated/`

## 数据模型

### 路径层次结构
```
~/.cc-switch/                    # 基础目录
├── config.json                  # cc-switch 配置文件
└── separated/                   # 分离配置目录
    ├── config-<encoded1>.json  # 独立配置文件
    ├── config-<encoded2>.json
    └── ...
```

### 编码映射示例
| 原始名称 | 编码后 | 说明 |
|----------|--------|------|
| `work` | `work` | 安全字符不变 |
| `Zhipu GLM` | `Zhipu%20GLM` | 空格编码为 `%20` |
| `my/config` | `my%2Fconfig` | 斜杠编码为 `%2F` |
| `test:dev` | `test%3Adev` | 冒号编码为 `%3A` |

## 测试和质量

### 单元测试覆盖
- 路径展开和解析
- 目录创建和权限处理
- 编码/解码正确性
- 边界情况处理

### 集成测试建议
- 不同操作系统的路径处理
- 特殊字符编码的完整性
- 大量文件操作的性能

### 错误处理策略
- **路径不存在**: 提供创建建议
- **权限不足**: 明确的权限错误信息
- **编码失败**: 详细的解码错误描述

## 常见问题 (FAQ)

### Q: 为什么需要编码配置名称？
A: 文件系统不支持某些字符（如 `/`, `:`, `*` 等），编码确保配置名称可以作为安全的文件名使用。

### Q: 编码后的文件名可读吗？
A: 编码使用标准的 URL 百分号编码，大部分工具都能正确显示和处理。

### Q: 如何处理不同操作系统的路径差异？
A: 使用 `std::path::PathBuf` 和 `dirs` 库提供的跨平台路径处理，确保兼容性。

### Q: 如果用户主目录无法获取怎么办？
A: `expand_tilde` 函数会返回明确的错误信息，建议用户检查环境设置。

## 相关文件列表

### 核心文件
- `mod.rs` - 模块声明
- `fs.rs` - 文件系统操作
- `encoding.rs` - 编码工具

### 依赖文件
- `../error.rs` - 错误类型定义

### 使用方文件
- `../config/manager.rs` - 配置管理（文件操作）
- `../commands/` - 各命令模块（路径处理）

## 性能优化

### 文件操作优化
- 使用原子写入（临时文件 + 重命名）
- 避免不必要的文件系统调用
- 批量操作时复用路径计算

### 编码优化
- 预编译编码字符集
- 避免重复编码相同字符串
- 使用高效的字符串处理

## 更新日志

### 2025-10-15 - 模块文档初始化
- 创建完整的工具模块文档
- 定义文件系统和编码接口
- 添加跨平台兼容性说明

### 历史版本
- 添加原子文件写入支持
- 优化编码字符集定义
- 完善错误处理机制