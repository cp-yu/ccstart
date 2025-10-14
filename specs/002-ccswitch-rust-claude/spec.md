# Feature Specification: ccstart - Claude Settings 配置管理工具

**Feature Branch**: `002-ccswitch-rust-claude`  
**Created**: 2025-10-14  
**Status**: Draft  
**Input**: 这是一个增加ccswitch功能的rust项目，属于是附属、增强的。利用claude --settings /home/yunxin/.claude/settings-packycode.json "$@"完成命令行快速切换操作。ccswitch的配置文件会放到 ~/.cc-switch的config.json中。而这里就是需要处理config.json文件。读取ccswitch中混合在一起的config文件，然后把他们分开到seperated文件夹中（ccstart init/update）。然后这样子就会有很多的config-<name>.json 文件。然后提供一个命令行补全功能，可以使用 ccstart <name> 实现claude --settings /home/yunxin/.claude/settings-packycode.json "$@"。然后ccstart <name> 还可以接受额外的输入，对应于"$@"。

## Clarifications

### Session 2025-10-14

- Q: config.json 的数据结构格式 → A: 嵌套对象结构 `{"claude": {"providers": {"name": {"settingsConfig": {...}}}}}` - 从 claude.providers 中提取每个 provider 的 settingsConfig 对象
- Q: 配置名称选择策略 → A: 使用 provider 的 `name` 字段作为配置名称，命令行调用和文件路径使用双引号包裹以处理空格和特殊字符，shell 补全也使用双引号包裹补全结果
- Q: 重复配置名称处理策略 → A: 使用后缀编号策略，当检测到重复的 name 时，自动为后续重复项添加数字后缀（如 "default", "default-2", "default-3"）
- Q: `ccstart init` 重新初始化行为 → A: 提示用户确认并清空重建 - 检测到 separated/ 目录已存在时，询问用户是否确认重新初始化，确认后删除该目录下所有文件并重新生成
- Q: Shell 补全脚本安装方式 → A: 提供 `ccstart completions <shell>` 命令输出补全脚本内容到 stdout，用户可以重定向到自己的 shell 配置文件中
- Q: 日志记录策略 → A: 标准错误输出（stderr）- 所有日志信息（包括调试、警告、错误）输出到 stderr，用户可通过 `2>/dev/null` 静默日志输出
- Q: Claude CLI 执行失败的退出码处理 → A: 透传退出码 - ccstart 直接返回 Claude CLI 的原始退出码，保持完整的错误信息传递链
- Q: 并发安全性策略 → A: 无锁设计 - 不实现并发控制机制，因为 Claude Code 会将 JSON 复制到内存，ccstart 处理速度快，极少出现冲突，且冲突仅发生在两个 ccstart 同时更改文件的罕见场景
- Q: 配置文件名特殊字符处理规则 → A: 允许空格保留在文件名中，其他特殊字符（如斜杠、冒号等）使用 URL 编码（percent-encoding）转义，命令行使用时必须用双引号包裹包含空格或特殊字符的配置名称
- Q: 配置列表显示命令 → A: 提供 `ccstart list` 子命令，列出所有可用配置名称（每行一个），方便用户查询和脚本使用

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 初始化配置分离 (Priority: P1)

用户首次使用 ccstart 工具时，需要将现有的 ccswitch 混合配置文件（~/.cc-switch/config.json）拆分成独立的配置文件，每个配置对应一个命名的 Claude settings 配置。

**Why this priority**: 这是整个工具的基础功能，没有配置分离就无法进行后续的配置切换操作。这是 MVP 的核心。

**Independent Test**: 可以通过运行 `ccstart init` 命令，检查是否在 ~/.cc-switch/separated/ 目录下生成了多个 config-<name>.json 文件来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户已安装 ccstart 且 ~/.cc-switch/config.json 存在，**When** 用户运行 `ccstart init`，**Then** 系统读取 config.json 并在 ~/.cc-switch/separated/ 目录下创建多个 config-<name>.json 文件
2. **Given** 用户已运行过 `ccstart init` 且 separated/ 目录存在，**When** 用户再次运行 `ccstart init`，**Then** 系统提示"配置已初始化，是否重新初始化？这将删除所有现有配置文件。(y/N)"，等待用户确认
3. **Given** 用户确认重新初始化，**When** 用户输入 'y' 或 'yes'，**Then** 系统删除 separated/ 目录下所有文件并重新从 config.json 生成配置文件
4. **Given** 用户拒绝重新初始化，**When** 用户输入 'n' 或直接回车，**Then** 系统取消操作并退出
5. **Given** ~/.cc-switch/config.json 不存在，**When** 用户运行 `ccstart init`，**Then** 系统显示错误信息，提示找不到源配置文件

---

### User Story 2 - 快速切换 Claude 配置 (Priority: P1)

用户需要通过简单的命令行指令快速切换不同的 Claude settings 配置，并能够传递额外的参数给 Claude 命令。

**Why this priority**: 这是工具的主要价值所在，提供便捷的配置切换能力。与 P1 的初始化功能共同构成完整的 MVP。

**Independent Test**: 可以通过运行 `ccstart <config-name> "some prompt"` 命令，验证是否正确调用了 `claude --settings <对应配置文件路径> "some prompt"` 来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户已完成配置初始化，**When** 用户运行 `ccstart packycode`，**Then** 系统使用 ~/.cc-switch/separated/config-packycode.json 作为 settings 文件启动 Claude
2. **Given** 用户已完成配置初始化，**When** 用户运行 `ccstart packycode "help me debug"`，**Then** 系统使用对应配置启动 Claude 并传递 "help me debug" 作为输入参数
3. **Given** 用户指定的配置名称不存在，**When** 用户运行 `ccstart nonexistent`，**Then** 系统显示错误信息，列出可用的配置名称

---

### User Story 3 - 命令行自动补全 (Priority: P2)

用户在输入 `ccstart` 命令时，可以通过 Tab 键自动补全可用的配置名称，提升使用体验。用户需要先通过 `ccstart completions <shell>` 命令获取补全脚本并安装到自己的 shell 配置中。

**Why this priority**: 这是用户体验增强功能，不影响核心功能的使用，但能显著提升效率和易用性。

**Independent Test**: 可以通过运行 `ccstart completions bash` 验证是否输出有效的 bash 补全脚本，然后在 shell 中 source 该脚本后输入 `ccstart <Tab>` 验证是否显示所有可用配置名称来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户运行 `ccstart completions bash`，**When** 命令执行，**Then** 系统输出 bash 格式的补全脚本到 stdout
2. **Given** 用户运行 `ccstart completions zsh`，**When** 命令执行，**Then** 系统输出 zsh 格式的补全脚本到 stdout
3. **Given** 用户已完成配置初始化且 shell 补全已安装，**When** 用户输入 `ccstart ` 并按 Tab 键，**Then** 系统显示所有可用的配置名称列表（使用双引号包裹）
4. **Given** 用户已输入部分配置名称，**When** 用户按 Tab 键，**Then** 系统自动补全匹配的配置名称或显示匹配的候选列表
5. **Given** 用户输入的前缀没有匹配项，**When** 用户按 Tab 键，**Then** 系统不显示任何补全建议

---

### User Story 4 - 更新配置 (Priority: P2)

用户修改了 ~/.cc-switch/config.json 后，需要重新同步到分离的配置文件中。

**Why this priority**: 这是维护功能，允许用户在修改源配置后保持一致性，但不是首次使用的必需功能。

**Independent Test**: 可以通过修改 config.json，运行 `ccstart update`，然后检查 separated/ 目录下的文件是否更新来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户已修改 ~/.cc-switch/config.json，**When** 用户运行 `ccstart update`，**Then** 系统重新读取 config.json 并更新 ~/.cc-switch/separated/ 目录下的所有配置文件
2. **Given** separated/ 目录下存在旧的配置文件，**When** 用户运行 `ccstart update`，**Then** 系统删除不再存在于 config.json 中的配置文件
3. **Given** config.json 中新增了配置项，**When** 用户运行 `ccstart update`，**Then** 系统在 separated/ 目录下创建对应的新配置文件

---

### User Story 5 - 列出所有配置 (Priority: P2)

用户需要查看当前所有可用的配置名称，以便选择合适的配置进行切换。

**Why this priority**: 这是便捷性功能，提升用户体验，但不是核心功能的必需部分。

**Independent Test**: 可以通过运行 `ccstart list` 命令，验证是否输出所有 separated/ 目录下的配置名称来独立测试。

**Acceptance Scenarios**:

1. **Given** 用户已完成配置初始化，**When** 用户运行 `ccstart list`，**Then** 系统输出所有可用的配置名称，每行一个
2. **Given** separated/ 目录下有 3 个配置文件（config-a.json, config-b.json, config-c.json），**When** 用户运行 `ccstart list`，**Then** 系统输出三行：a、b、c
3. **Given** separated/ 目录不存在或为空，**When** 用户运行 `ccstart list`，**Then** 系统显示"未找到配置，请先运行 'ccstart init'"
4. **Given** 配置名称包含空格或特殊字符，**When** 用户运行 `ccstart list`，**Then** 系统输出的配置名称使用双引号包裹

---

### Edge Cases

- 当 ~/.cc-switch 目录不存在时，系统如何处理？
- 当 config.json 格式不正确或为空时，系统如何处理？
- 当 separated/ 目录权限不足无法写入时，系统如何处理？
- 当配置名称包含需要 URL 编码的特殊字符（如斜杠、冒号）后，生成的文件名是否能正确反向解析？
- 当配置名称包含空格或特殊字符时，shell 补全是否正确使用双引号包裹？
- 当多个 provider 具有相同的 name 时，如何确保生成的文件名唯一？
- 当 Claude CLI 不在系统 PATH 中时，系统如何处理？
- 当配置文件路径包含空格或特殊字符时，命令行参数如何正确传递？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统必须能够读取 ~/.cc-switch/config.json 文件并解析其 JSON 结构，特别是 `claude.providers` 对象
- **FR-002**: 系统必须能够从 config.json 的 `claude.providers` 中提取每个 provider 的 settingsConfig 对象，使用 provider 的 `name` 字段作为配置名称，并将其保存为独立的 config-<name>.json 文件存储在 ~/.cc-switch/separated/ 目录。当检测到重复的配置名称时，系统必须自动为后续重复项添加数字后缀（如 "default-2", "default-3"）以确保唯一性
- **FR-003**: 系统必须提供 `init` 子命令用于初始化配置分离。当 separated/ 目录已存在时，必须提示用户确认是否重新初始化，确认后删除该目录下所有文件并重新生成
- **FR-004**: 系统必须提供 `update` 子命令用于更新已分离的配置文件
- **FR-005**: 系统必须支持 `ccstart <name>` 命令格式来选择并使用指定的配置，配置名称包含空格或特殊字符时必须使用双引号包裹
- **FR-006**: 系统必须能够将 `ccstart <name>` 后的所有额外参数原样传递给 Claude CLI，并正确处理路径中的空格和特殊字符（使用双引号包裹）
- **FR-007**: 系统必须在配置名称不存在时显示友好的错误信息和可用配置列表
- **FR-008**: 系统必须提供 shell 补全脚本支持（至少支持 bash 和 zsh），补全结果必须使用双引号包裹以正确处理包含空格或特殊字符的配置名称
- **FR-009**: 系统必须在源配置文件不存在时给出明确的错误提示
- **FR-010**: 系统必须在配置文件格式错误时给出明确的错误提示和修复建议
- **FR-011**: 系统必须确保生成的配置文件名安全，允许空格保留在文件名中，对其他特殊字符（如斜杠 `/`、冒号 `:` 等文件系统不安全字符）使用 URL 编码（percent-encoding）转义
- **FR-012**: 系统必须在 separated/ 目录不存在时自动创建该目录
- **FR-013**: 系统必须将所有日志信息（包括调试、警告、错误）输出到标准错误输出（stderr），确保正常输出（stdout）仅用于功能性输出（如补全脚本），便于用户通过 `2>/dev/null` 控制日志显示
- **FR-014**: 当 `ccstart <name>` 调用 Claude CLI 时，系统必须透传 Claude CLI 的原始退出码，确保调用方能够正确识别 Claude CLI 的执行状态
- **FR-015**: 系统必须提供 `list` 子命令，列出 separated/ 目录下所有可用的配置名称（每行一个），配置名称包含空格或特殊字符时使用双引号包裹

### Key Entities

- **配置集合 (Config Collection)**: 存储在 ~/.cc-switch/config.json 中的混合配置数据，结构为 `{"claude": {"providers": {"provider-name": {"settingsConfig": {...}}}}}`，包含多个命名的 Claude provider 配置
- **Provider 配置**: config.json 中 `claude.providers` 下的每个对象，包含 id、name 和 settingsConfig 字段
- **独立配置 (Separated Config)**: 从 provider 的 settingsConfig 中提取的单个配置对象，存储为 ~/.cc-switch/separated/config-<name>.json 文件，格式与 Claude CLI 的 settings 文件格式一致
- **配置名称 (Config Name)**: provider 的 `name` 字段，用于标识和选择特定配置，用户通过此名称调用配置。当名称包含空格或特殊字符时，必须使用双引号包裹

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 用户可以在 5 秒内完成配置初始化操作（运行 `ccstart init`）
- **SC-002**: 用户可以通过 `ccstart <name>` 命令在 1 秒内启动 Claude 并使用指定配置
- **SC-003**: 命令行补全功能能够在 0.5 秒内显示所有可用配置名称
- **SC-004**: 系统能够正确处理至少 50 个不同的配置项而不出现性能下降
- **SC-005**: 用户在使用过程中遇到的配置相关错误能够通过清晰的错误信息在 2 分钟内自行解决
- **SC-006**: 配置更新操作（`ccstart update`）能够在 3 秒内完成同步

## Assumptions

- 假设用户已安装 Claude CLI 工具且可通过 `claude` 命令访问
- 假设 ~/.cc-switch/config.json 的格式为标准 JSON，结构为 `{"claude": {"providers": {...}}}`，其中 providers 对象包含多个 provider 配置
- 假设每个 provider 配置包含 settingsConfig 字段，该字段的内容符合 Claude CLI settings 文件格式
- 假设用户使用的是 Linux 或 macOS 系统（基于路径格式 ~/.cc-switch）
- 假设配置文件大小合理（单个配置文件不超过 10MB）
- 假设用户具有 ~/.cc-switch 目录的读写权限
- 假设 shell 补全脚本需要用户手动安装到对应的 shell 配置文件中
- 假设用户不会同时运行多个 ccstart init/update 命令修改配置文件（无并发控制机制）
