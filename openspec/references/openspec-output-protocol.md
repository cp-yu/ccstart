# Optimizer Output Protocol

Return exactly one of two responses.

## Response A: No Optimization Needed

```
No optimization opportunities found
```

Use this when:
- Code structure is already clean with no meaningful duplication, nesting, or indirection problems.
- The change is a pure deletion, rename, or parameter removal.
- All failedDirections cover every plausible optimization strategy.
- You cannot understand code intent clearly enough to propose safe improvements.

## Response B: Search/Replace Blocks

Return one or more blocks in this exact format:

```text
<!-- Code Smell: <Duplication | Long Method | Shallow Module | Feature Envy | Primitive Obsession | Deep Nesting | Dead Code> -->
<<<PATH: relative/path/to/file.ts
<<<SEARCH
exact old text
===
replacement new text
>>>REPLACE
```

Multiple blocks are separated by a blank line.
Every block MUST include exactly one preceding `<!-- Code Smell: <type> -->` annotation using one of these values: `Duplication`, `Long Method`, `Shallow Module`, `Feature Envy`, `Primitive Obsession`, `Deep Nesting`, `Dead Code`.

## Search/Replace Constraints

- Each block MUST target exactly one existing file.
- The SEARCH payload MUST be specific enough to match exactly one location in the target file. Include enough surrounding context (3-5 lines before and after the changed region) to guarantee uniqueness.
- Use actual whitespace from the file (tabs, spaces, trailing). The main agent will try exact match first, then whitespace-normalized.
- A block whose SEARCH matches zero or multiple locations will be rejected and MUST be regenerated.
- All blocks together MUST be internally consistent - applying them in order MUST NOT produce conflicts.
- Do NOT number or index blocks. Raw blocks only.

## Failed Directions Protocol

### Reading Failed Directions

Before proposing any optimization, inspect failedDirections. Each entry is a natural-language summary of a previously attempted strategy that caused FAIL_NEEDS_REMEDIATION on re-verify. Examples:

- "extract shared validation logic from auth.ts and user.ts into common/validators.ts"
- "flatten the middleware chain in api/router.ts from 5 layers to 2"
- "consolidate error handling in service/order.ts and service/payment.ts"

### Avoiding Repeated Failures

- You MUST NOT propose an optimization whose strategy is the same as or substantially similar to any entry in failedDirections.
- "Substantially similar" means: same files targeted, same type of structural change, same abstraction boundary.
- If all plausible optimization strategies are already in failedDirections, return No optimization opportunities found.

### Recording New Failures

You do NOT record failures yourself. The top-level agent appends to failedDirections after a speculative re-verify returns FAIL_NEEDS_REMEDIATION. Your responsibility is only to read and respect the existing list.

## Edge Cases

### Empty or Single-File Changes

For changes affecting only one small file with straightforward logic: check for the improvements listed in Optimization Principles. If the file is already clean, return No optimization opportunities found. Do not manufacture improvements where none exist.

### Pure Deletions or Renames

Return No optimization opportunities found immediately. These changes have no meaningful optimization surface.

### Code Already Optimized in Prior Cycle

If failedDirections is non-empty and Phase 1 passed, all previously attempted optimizations have already been tried and reverted. Return No optimization opportunities found.

### Ambiguous Improvement

If a potential improvement could affect behavior (e.g., reordering side-effectful calls, changing error propagation), do NOT propose it. Optimizations MUST be provably behavior-preserving from static analysis alone.

### Cross-File Refactoring

You MAY propose blocks that span multiple files (e.g., extracting a shared function from two files into a third). Ensure:
- Every block targets an existing tracked base scope file.
- No file is created or deleted (the extraction target MUST already exist).
- All blocks together form a coherent, atomic change.

### Subagent Timeout

If the main agent reports your response took too long, it will discard your output and record ABORTED_UNSAFE. Produce your analysis and blocks efficiently. Focus on specific regions with improvement potential - do not enumerate every line of every file.