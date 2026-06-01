# Bootstrap Review

在提升为正式 OPSX 文件之前，先审阅当前映射出的架构。

该文件由 evidence.yaml 和 domain-map/*.yaml 派生；任一内容变化后，都需要重新运行 `openspec bootstrap validate` 生成新的 review。

## Refresh Scope

- 策略：git-diff
- 锚点提交：e2422f467b68f63ae01b97e709d582e446f77bfc
- 原因：git 锚点可用，但未发现需要映射到 code-map 的源码路径变化。
- 受影响 domains：dom.cache, dom.cli, dom.provider
- 保留的基线节点数：0

## Delta Summary

- ADDED：0 个节点，0 条关系
- MODIFIED：0 个节点，0 条关系
- REMOVED：0 个节点，0 条关系

## Domain Checklist

- [x] dom.cache — 1 个 capability, confidence: high
- [x] dom.cli — 1 个 capability, confidence: high
- [x] dom.provider — 1 个 capability, confidence: high

## Candidate Specs

- 当前不会写入 candidate spec
- 除非新增 capability 缺少 formal spec，否则现有 formal specs 继续作为唯一真相来源。
- 保留已有 spec：openspec/specs/cache/spec.md
- 保留已有 spec：openspec/specs/cli/spec.md
- 保留已有 spec：openspec/specs/provider/spec.md

## Validation

- [x] Review 内容与当前 candidate 输出一致
- [x] 引用完整性校验通过
- [x] Code-map 中的路径都存在于磁盘上
- [x] Candidate spec 集合符合当前 bootstrap 模式约束
- [x] Refresh delta 与当前 formal OPSX 基线一致
- [x] Candidate spec 通过 OpenSpec 校验
- [x] Domain 边界与预期心智模型一致
