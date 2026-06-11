## Context

ccstart 当前无参数时显示 help 文本，cxstart 无参数时显示 usage 提示，`ccstart codex` 无参数时同样显示 help。三个入口都缺少交互式选择能力——用户必须事先知道配置名称才能启动。

项目定位为轻量启动器，二进制体积敏感（`opt-level = "z"` + LTO），现有拼音匹配逻辑已在 `utils/pinyin.rs` 中实现。

## Goals / Non-Goals

**Goals:**
- 三个无参数入口进入内联 TUI 选择器
- vim 风格导航 + 模糊搜索（含拼音首字母匹配）
- 最小化新依赖对二进制体积的影响

**Non-Goals:**
- 全屏 TUI 应用（不启用 alternate screen）
- 多列/预览/复杂布局
- 异步运行时

## Decisions

### 1. 使用 crossterm 手写，而非 skim/ratatui

**选择**: crossterm raw mode + 自定义事件循环

**备选方案**:
- skim：开箱即用但拉入 tokio，二进制膨胀 ~2MB；自定义拼音 scorer 需 hack
- ratatui + nucleo：全屏风格偏重，nucleo 不支持拼音

**理由**: 内联列表渲染逻辑简单（输入行 + N 行候选 + 高亮），crossterm 纯 Rust 无 async 依赖，约 200-300KB 体积增量可接受。拼音匹配直接复用已有 `pinyin_initials`。

### 2. 双模式状态机：Normal / Filter

**选择**: 按 `f` 从 Normal 进入 Filter，`Esc` 回到 Normal（保留搜索内容）

**理由**: 避免 vim motion 键（j/k/g/G）与搜索输入冲突。用户明确切换意图后才开始输入搜索字符。

### 3. Filter 模式下导航使用箭头键和 Ctrl-N/P

**选择**: Filter 模式中 j/k 作为普通字符输入，导航依赖 `↑/↓/Ctrl-P/Ctrl-N`

**理由**: 搜索框中 j/k 是高频字符，强占为导航会破坏输入体验。

### 4. Kill ring 使用单 slot 实现

**选择**: 会话级单 slot 存储 `Ctrl-U`/`Ctrl-K` 删除的文本，`Ctrl-Y` 粘贴

**理由**: 完整 ring 对此场景过度工程。单次选择操作中 kill-yank 只需一个槽位。

### 5. 内联渲染策略

**选择**: 不启用 alternate screen，在当前终端位置渲染列表，退出时清除占用行

**理由**: fzf 风格交互——轻量、即时、不打断终端上下文。

## Risks / Trade-offs

- [crossterm 终端兼容性] → crossterm 已覆盖主流终端（Windows Terminal、alacritty、iTerm2、kitty）；WSL2 下 crossterm raw mode 已有广泛使用
- [二进制体积增长 200-300KB] → 对 CLI 启动器可接受，仍远小于 skim/ratatui 方案
- [gg 双键检测需要超时或状态记忆] → 使用 pending-g 状态：第一个 `g` 设置标志，第二个 `g` 在短窗口内到达则执行跳顶，否则重置
