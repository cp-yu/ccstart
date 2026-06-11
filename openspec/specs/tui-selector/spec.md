# tui-selector Specification

## Purpose
此规约记录变更 add-tui-selector 引入的行为，请在后续同步或归档前补全正式 Purpose。
## Requirements
### Requirement: 内联选择器界面
系统 SHALL 提供内联模糊搜索选择器，在当前终端位置渲染列表，支持搜索过滤和 vim 风格导航，用户选中后返回所选项名称。

#### Scenario: 正常渲染和选择
- **WHEN** 调用 `tui::select(&items, title)` 且 items 非空
- **THEN** 终端进入 raw mode，显示标题行、候选列表（高亮当前项），等待用户交互

#### Scenario: 确认选择
- **WHEN** 用户在高亮某项时按 `Enter`
- **THEN** 函数返回 `Some(selected_name)`，终端恢复正常状态并清除已占用行

#### Scenario: 取消选择
- **WHEN** 用户按 `Esc` 或 `Ctrl-C`（Normal 模式下）
- **THEN** 函数返回 `None`，终端恢复正常状态并清除已占用行

### Requirement: Normal 模式导航
系统 SHALL 在 Normal 模式下支持 vim 风格的列表导航键位。

#### Scenario: j/k 上下移动
- **WHEN** 用户按 `j` 或 `↓`
- **THEN** 高亮移动到下一项（到底部时不循环）

#### Scenario: gg 跳转顶部
- **WHEN** 用户按 `g` 后再按 `g`
- **THEN** 高亮跳转到列表第一项

#### Scenario: G 跳转底部
- **WHEN** 用户按 `G`
- **THEN** 高亮跳转到列表最后一项

#### Scenario: f 进入 Filter 模式
- **WHEN** 用户按 `f`
- **THEN** 界面切换到 Filter 模式，显示搜索输入框

### Requirement: Filter 模式搜索
系统 SHALL 在 Filter 模式下接受字符输入作为搜索查询，实时过滤候选列表。

#### Scenario: 字符输入过滤
- **WHEN** 用户输入字符
- **THEN** 候选列表实时更新为匹配搜索内容的子集

#### Scenario: 箭头键和 Ctrl-N/P 导航
- **WHEN** 用户按 `↑`/`↓` 或 `Ctrl-P`/`Ctrl-N`
- **THEN** 在过滤后的列表中移动高亮

#### Scenario: Ctrl-U 删除光标前内容
- **WHEN** 用户按 `Ctrl-U`
- **THEN** 搜索框中光标前的所有内容被删除

#### Scenario: Ctrl-K 删除到行尾
- **WHEN** 用户按 `Ctrl-K`
- **THEN** 搜索框中光标到行尾的内容被删除，删除内容存入 kill slot

#### Scenario: Ctrl-Y 粘贴
- **WHEN** 用户按 `Ctrl-Y`
- **THEN** kill slot 中的内容被插入到光标位置

#### Scenario: Ctrl-C 清空搜索
- **WHEN** 用户按 `Ctrl-C`（Filter 模式下）
- **THEN** 搜索框内容被清空，列表恢复完整显示，留在 Filter 模式

#### Scenario: Esc 回到 Normal 模式
- **WHEN** 用户按 `Esc`（Filter 模式下）
- **THEN** 回到 Normal 模式，搜索内容和过滤结果保持不变

### Requirement: 模糊搜索含拼音匹配
系统 SHALL 对搜索查询执行大小写不敏感的子串匹配，当查询为纯 ASCII 字母时额外执行拼音首字母匹配。

#### Scenario: 子串匹配
- **WHEN** 搜索输入为 "pack"
- **THEN** 名称中包含 "pack"（不区分大小写）的项被匹配

#### Scenario: 拼音首字母匹配
- **WHEN** 搜索输入为纯 ASCII 字母 "xc"
- **THEN** 拼音首字母为 "xc" 的中文名称（如"小丑"）被匹配

#### Scenario: 合并去重
- **WHEN** 某项同时满足子串匹配和拼音匹配
- **THEN** 结果中该项只出现一次，保持原始排序

