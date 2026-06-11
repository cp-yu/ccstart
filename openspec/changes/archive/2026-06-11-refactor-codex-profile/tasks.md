### Task 1: 添加 pinyin crate 依赖并实现首字母提取

**Goal**: 引入拼音转换能力，实现从中文字符串提取拼音首字母序列

**Files**:
- Modify: `Cargo.toml`
- Create: `src/utils/pinyin.rs`
- Modify: `src/utils/mod.rs`

**Requirements**:
- 添加轻量 pinyin crate 依赖
- 实现 `pinyin_initials(name: &str) -> String` 函数，提取每个中文字符的拼音首字母
- ASCII 字符保留原样（小写），非 ASCII 非中文字符忽略

#### Checks

- [x] C1 中文名提取首字母正确
  - Verifies: `specs/pinyin-resolve/spec.md` / Requirement "拼音首字母匹配" / Scenario "唯一首字母匹配"
  - Command: `cargo test pinyin_initials`
  - Expect: "小丑" → "xc", "钟阮" → "zr", "packyapi" → "packyapi"

### Task 2: 实现渠道名称解析逻辑

**Goal**: 实现精确匹配 + 拼音首字母 fallback 的解析器

**Files**:
- Create: `src/commands/codex_resolve.rs`
- Modify: `src/commands/mod.rs`

**Requirements**:
- 精确匹配优先
- 仅当输入全为 ASCII 字母时尝试拼音首字母匹配
- 唯一命中 → 返回匹配名称
- 多个命中 → 返回碰撞候选列表（调用方负责展示）
- 无命中 → 返回 None

#### Checks

- [x] C2 精确匹配优先于拼音
  - Verifies: `specs/pinyin-resolve/spec.md` / Requirement "精确匹配优先" / Scenario "精确名称直接命中"
  - Command: `cargo test resolve_exact`
  - Expect: 输入 "小丑" 精确返回 "小丑"

- [x] C3 拼音首字母唯一匹配
  - Verifies: `specs/pinyin-resolve/spec.md` / Requirement "拼音首字母匹配" / Scenario "唯一首字母匹配"
  - Command: `cargo test resolve_pinyin_unique`
  - Expect: 输入 "xc" 匹配到 "小丑"

- [x] C4 拼音碰撞列出候选
  - Verifies: `specs/pinyin-resolve/spec.md` / Requirement "拼音首字母匹配" / Scenario "首字母碰撞列出候选"
  - Command: `cargo test resolve_pinyin_collision`
  - Expect: 多个同首字母渠道时返回碰撞候选列表

### Task 3: 实现 codex profile 缓存模块

**Goal**: 将 TOML 配置写入 ~/.codex/ 下的 profile 文件，使用 Read-Through Cache

**Files**:
- Create: `src/config/codex_cache.rs`
- Modify: `src/config/mod.rs`

**Requirements**:
- profile 文件名: `ccstart-<sha256(channel_name)[..8]>.config.toml`
- SHA256 内容哈希比较决定是否写入
- 原子写入（临时文件 + rename）
- 返回 CacheResult 枚举 (Unchanged/Created/Updated)
- 清理 `ccstart-*` 前缀的过期 profile 文件

#### Checks

- [x] C5 新 profile 文件正确创建
  - Verifies: `specs/codex-profile-cache/spec.md` / Requirement "Profile 文件 Read-Through Cache" / Scenario "Profile 首次创建"
  - Command: `cargo test codex_cache_create`
  - Expect: 文件写入 ~/.codex/ccstart-<hash>.config.toml，内容为原始 TOML

- [x] C6 内容未变时不写入
  - Verifies: `specs/codex-profile-cache/spec.md` / Requirement "Profile 文件 Read-Through Cache" / Scenario "内容相同时复用"
  - Command: `cargo test codex_cache_unchanged`
  - Expect: 返回 Unchanged，文件 mtime 不变

- [x] C7 过期 profile 清理
  - Verifies: `specs/codex-profile-cache/spec.md` / Requirement "过期 Profile 清理" / Scenario "同步时删除过期文件"
  - Command: `cargo test codex_cache_cleanup`
  - Expect: 不在数据库中的 ccstart-* 文件被删除

### Task 4: 重构 codex dispatch 使用 -p 启动

**Goal**: 用 profile 文件替代逐字段 -c 注入

**Files**:
- Modify: `src/commands/codex.rs`

**Requirements**:
- 集成渠道名称解析（Task 2）
- 调用 profile 缓存确保文件就绪（Task 3）
- 执行: `OPENAI_API_KEY=xxx codex -p <profile_name> [user_args...]`
- 碰撞时打印候选列表并退出
- 保持 list_channels() 不变

#### Checks

- [x] C8 使用 -p 参数启动 codex
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Profile 模式启动" / Scenario "正常 profile 启动"
  - Command: `cargo test codex_dispatch_profile`
  - Expect: Command 构建中包含 `-p ccstart-<hash>` 而非多个 `-c`

- [x] C9 环境变量注入 API key
  - Verifies: `specs/codex-dispatch/spec.md` / Requirement "Profile 模式启动" / Scenario "正常 profile 启动"
  - Command: `cargo test codex_dispatch_env`
  - Expect: Command 环境包含 OPENAI_API_KEY

- [x] C10 拼音碰撞时退出并列出候选
  - Verifies: `specs/pinyin-resolve/spec.md` / Requirement "拼音首字母匹配" / Scenario "首字母碰撞列出候选"
  - Evidence: 代码审查 `codex.rs` 中碰撞处理分支
  - Expect: 打印候选列表，返回退出码 1

### Task 5: 实现 cxstart argv[0] 快捷入口

**Goal**: 检测 argv[0] 为 cxstart 时隐含 codex 子命令

**Files**:
- Modify: `src/main.rs`

**Requirements**:
- `basename(argv[0]) == "cxstart"` 时将参数作为 codex channel + passthrough 处理
- `cxstart list` → codex list_channels
- `cxstart <channel> [args...]` → codex run
- 无参数时显示 codex 帮助

#### Checks

- [x] C11 argv[0] 为 cxstart 时路由到 codex
  - Verifies: `specs/cxstart-alias/spec.md` / Requirement "argv[0] 路由" / Scenario "cxstart 隐含 codex 调度"
  - Command: `cargo test cxstart_routing`
  - Expect: 模拟 argv[0]="cxstart" 时进入 codex 路径

- [x] C12 CI release 产出 cxstart 副本
  - Verifies: `specs/cxstart-alias/spec.md` / Requirement "发布产物包含 cxstart" / Scenario "CI 构建输出 cxstart"
  - Evidence: `.github/workflows/release.yml` 包含 cxstart 复制步骤
  - Expect: release 产物包含 cxstart-linux-x64 和 cxstart-windows-x64.exe

### Task 6: 补全支持拼音首字母

**Goal**: shell 动态补全同时匹配中文名和拼音首字母前缀

**Files**:
- Modify: `src/main.rs` (`codex_channel_completer`)

**Requirements**:
- 保持原有中文前缀匹配
- 输入为 ASCII 时额外匹配拼音首字母前缀
- 返回原始中文名作为候选（非拼音）

#### Checks

- [x] C13 拼音前缀返回中文候选
  - Verifies: `specs/codex-completion/spec.md` / Requirement "拼音首字母补全" / Scenario "拼音前缀匹配返回中文名"
  - Command: `cargo test completion_pinyin`
  - Expect: 输入 "xc" 返回候选 "小丑"

### Task 7: update 命令同步 codex profile

**Goal**: ccstart update 对称处理 codex profile 文件

**Files**:
- Modify: `src/commands/update.rs`

**Requirements**:
- 遍历所有 codex channel 同步 profile
- 清理不在数据库中的 `ccstart-*` profile 文件
- 报告 Created/Updated/Unchanged/Removed 统计

#### Checks

- [x] C14 update 同步 codex profile
  - Verifies: `specs/codex-profile-cache/spec.md` / Requirement "Profile 文件 Read-Through Cache" / Scenario "Profile 内容变更时更新"
  - Command: `cargo test update_syncs_codex`
  - Expect: update 输出包含 codex profile 同步统计

- [x] C15 update 清理过期 profile
  - Verifies: `specs/codex-profile-cache/spec.md` / Requirement "过期 Profile 清理" / Scenario "同步时删除过期文件"
  - Command: `cargo test update_cleans_codex`
  - Expect: 数据库中不存在的 ccstart-* 文件被删除
