# Boundary Commit Message Template

Use this built-in format when `git.commitMessage.boundary` is not configured.

Subject:

```text
<type>(<scope>): <中文标题>
```

Body:

```text
## Why
[业务背景] <why from proposal.md>
[技术决策] <decision from design.md when present>

## Changes
- `<file-path>`: <why this file changed>
```

Footer:

```text
Implementation: <base>..<head> (carried by <commits>)
```

Rules:
- Build `## Why` from the archived `proposal.md` and `design.md` (when present).
- Build `## Changes` from `git diff --name-only <base>..<head>` file list cross-referenced with archived `tasks.md` Files/Goal and `opsx-delta.yaml`.
- List all files from the diff; if a file is not mentioned in archived artifacts, state that explicitly.
- The `<base>` is the prior change boundary (prior archive/boundary commit); the agent infers it from git history.
- The `Implementation:` footer lists the effective diff range and the commits carrying that diff (including `wip: opt-*` checkpoint commits and any just-created normal implementation commit).
- Use `git commit -F -` to pass the message.
- Always create this commit with `git commit --allow-empty`; it may be intentionally empty when all implementation diff is carried by checkpoint commits.