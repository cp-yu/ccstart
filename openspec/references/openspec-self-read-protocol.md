# Optimizer Self-Read Protocol

Read the optimization context yourself in this order:

1. Validate changeName, changeDir, and projectRoot.
2. Read changeDir/.verify-result.json and extract result, issues, summary, verificationContext.evidenceFiles, and optimization.failedDirections.
3. Read change artifacts from changeDir: proposal.md, specs/*/spec.md, and design.md when present.
4. Read projectRoot/openspec/config.yaml for optimization.enabled and optimization.optRetries when present.
5. Resolve originalBranch for branch-aware scope anchoring:
   - First read changeDir/.apply-isolation.json and use originalBranch when present.
   - If missing, run git symbolic-ref refs/remotes/origin/HEAD --short and strip the origin/ prefix.
   - If still unresolved, ask the user for the original branch before selecting optimization scope.
6. Run git diff <originalBranch>...HEAD --name-only from projectRoot to build the base scope. Treat this name-only output as a navigation list, not behavior evidence.
7. Read candidate implementation files from verificationContext.evidenceFiles and base scope, excluding spec files, design documents, tasks files, config files, and untracked paths.
8. Apply Dependency Expansion (One Hop) before deciding whether an optimization opportunity exists.

## Dependency Expansion (One Hop)

Expand the base scope by one hop to understand safe optimization context:

1. imports - parse direct import/require/from references from base scope files and resolve project-local targets.
2. callers - identify exported names from base scope files and search direct callers in tracked project files.
3. OPSX relations - when project.opsx.relations.yaml exists, find one-hop depends_on / relates_to neighbors for nodes mapped to base scope files.

Expansion stops after one hop. Do not recursively expand expansion candidates.

Filter expansion candidates before reading them:
- Use path.relative(projectRoot, candidate) and discard paths whose relative form starts with ..
- Reuse gitignore parsing when available.
- Exclude ignored directories: node_modules, dist, build, .git.

If relations are missing, continue with imports and callers only; do not fail.

Expansion candidates MUST NOT be patch targets. Search/Replace PATH values MUST remain inside base scope files only. affectedFileHashes MUST include base scope files only; expansion candidates are read-only context.