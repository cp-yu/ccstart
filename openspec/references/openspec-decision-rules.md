# Optimization Decision Rules

## What to Improve

Seek these improvements in priority order:

1. **Lower duplication** - Extract repeated logic into shared functions, deduplicate validation, consolidate error handling patterns.
   - Code smell indicators: identical logic blocks in two or more locations, copy-pasted validation or transformation logic, repeated error handling patterns.
2. **Simpler structure** - Flatten unnecessary nesting, reduce indirection layers, replace over-engineered abstractions with direct code.
   - Code smell indicators: wrappers that only forward calls, abstractions with one trivial implementation, configuration objects that hide direct values without adding behavior.
3. **Clearer control flow** - Prefer early returns over deep conditionals, reduce cyclomatic complexity, make happy path obvious.
   - Code smell indicators: methods longer than 30 lines, conditional nesting deeper than three levels, return paths hidden inside nested branches.
4. **Better locality** - Move related code closer together, keep data and its operations in the same module, reduce cross-module coupling.
   - Code smell indicators: Feature Envy where a method mainly operates on another class's data, getter chains, logic placed away from the data owner.
5. **Remove dead weight** - Eliminate unused imports, unreachable branches, commented-out code, redundant type assertions.
   - Code smell indicators: unused imports or locals, unreachable conditional branches, commented-out code, redundant type assertions that do not change type safety.
6. **Break long methods** - Split methods longer than 30 lines by extracting private helper methods that each do one thing while keeping the public method signature, parameters, and return value unchanged.
   - Code smell indicators: a method exceeds 30 lines, mixes validation with transformation and side effects, or needs comments to explain internal phases.
   - Refactoring pattern: extract private helper methods with descriptive names; do not change the public interface.
7. **Deepen shallow modules** - Replace shallow modules with deeper abstractions when the public API is broader or more parameter-heavy than the behavior it hides.
   - Evaluation criteria: method count, parameter complexity, and hidden internal complexity.
   - Action strategy: merge related shallow modules, push complexity behind the implementation, simplify the public API.
8. **Eliminate primitive obsession** - Replace domain-heavy primitive strings or numbers with value objects when validation or domain rules repeat.
   - Candidate types: Email, money/currency, date ranges, identifiers.
   - Benefits: validation is encapsulated once, domain concepts become type-safe, and call sites become self-documenting.

## What NOT to Touch

- Spec files, design documents, tasks files - structural documents, not implementation.
- Configuration files (config.yaml, package.json, tsconfig.json).
- Test files unless a test is structurally identical to production logic being deduplicated.
- Files with no issues in Phase 1 and no structural improvement opportunity visible on inspection.
- Any change that alters observable behavior, even trivially.

## Constraint Checklist

Before finalizing any Search/Replace block, verify:
- [ ] Targets an existing tracked base scope file from verificationContext.evidenceFiles or base scope
- [ ] Does not create, delete, rename, or move any file
- [ ] Preserves all existing behavior (same inputs -> same outputs, same side effects)
- [ ] Does not touch spec, design, tasks, or config files
- [ ] The improvement is structural, not cosmetic (no variable renames, reformatting, comment rewording)