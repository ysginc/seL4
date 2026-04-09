# Rust Rewrite Charter (Phase 1)

## Purpose

This charter defines scope, safety constraints, target architecture, and governance for a Phase 1 Rust rewrite effort for seL4 kernel-adjacent code. It is intentionally conservative: preserve behavior first, then broaden platform and verification coverage.

## 1) In-Scope Components (Phase 1)

The following components are in scope for initial Rust translation or Rust-first reimplementation where semantics are unchanged:

- **Kernel source internals under `src/`** that are architecture-agnostic or have a bounded architecture surface suitable for one-platform bring-up.
- **Kernel internal headers under `include/`** used by translated `src/` paths, including type/constant definitions and internal helper interfaces needed to preserve control/data-flow semantics.
- **Selected `libsel4` generator outputs** where generated interface bindings are required to compile and exercise the rewritten paths end-to-end.
- **Build integration glue** required to compile mixed C/Rust artifacts for the reference target, including symbol linkage, section placement, and panic/abort strategy consistent with kernel constraints.

### Scope Guardrails

- Rewrites must be behavior-preserving relative to current C implementation and configuration for the reference target.
- New abstractions are acceptable only when they reduce unsafety surface or make invariants explicit without altering externally observable behavior.
- Any scope expansion beyond the items above requires explicit sign-off in governance review.

## 2) Out-of-Scope Components (Phase 1)

The following are explicitly deferred:

- **Formal proofs and proof artifacts** (Isabelle/HOL or related proof pipeline updates).
- **Non-reference / niche platforms** beyond the initial architecture matrix target.
- **Legacy debug, tracing, and diagnostic paths** that are not required for boot and core functional parity on the reference platform.
- **Performance retuning and micro-optimizations** beyond regressions that block acceptance criteria.
- **API redesign** for user-visible interfaces, including capability API semantics and syscall contract changes.

## 3) Safety Policy

Rust safety is the default. `unsafe` is an exception and must be justified.

### Where `unsafe` Is Allowed

`unsafe` is permitted only for:

- **FFI boundaries** (calls to or from C/assembly, externally defined symbols).
- **Memory-mapped I/O and raw pointer manipulation** required by kernel low-level operations.
- **Context switch / trap / interrupt boundary shims** where ABI/register semantics cannot be expressed safely.
- **Linker/section and boot-time initialization primitives** requiring compiler or linker contracts.

### Unsafe Documentation Requirements

Every `unsafe` block/function/trait impl must include a concise `SAFETY:` comment documenting:

1. **Invariant(s)** that must hold before entry.
2. **Why the operation is sound** under those invariants.
3. **Ownership/aliasing guarantees** (including mutability exclusivity assumptions).
4. **Concurrency/ordering assumptions** (interrupt state, lock state, atomic ordering).
5. **Lifetime/validity bounds** for pointers/references crossing the unsafe boundary.

### Additional Safety Constraints

- Prefer the smallest possible unsafe region (block-level over function-level when feasible).
- `unsafe fn` must describe caller obligations in API docs.
- No `static mut` unless encapsulated behind a reviewed synchronization/access discipline.
- Introduce targeted checks/assertions at unsafe boundaries where cost is acceptable for debug builds.

## 4) Initial Target Architecture Matrix

Phase 1 starts with one reference platform:

| Architecture | Platform | Status | Notes |
|---|---|---|---|
| x86_64 | pc99 | **Primary (Phase 1)** | Sole required bring-up and parity target for this phase. |

Planned follow-on architectures are tracked separately and are not acceptance blockers for this charter.

## 5) Acceptance Criteria

## 5.1 “Feature Parity” Criteria

A Phase 1 rewrite is considered at feature parity for the reference target when all of the following hold:

- **Boot parity:** kernel boots successfully on x86_64 `pc99` using supported build configuration(s).
- **Functional parity:** existing baseline tests for covered subsystems pass with no semantic regressions.
- **Interface parity:** externally observable syscall and capability behavior for covered paths remains unchanged.
- **Stability parity:** no increase in crash/panic frequency in parity test runs relative to baseline.
- **Safety accounting:** all introduced `unsafe` sites are documented and reviewed per policy.

## 5.2 “Ready for Verification Handoff” Criteria

The rewrite is ready for verification handoff when feature parity is met and:

- **Traceability:** C-to-Rust mapping exists for rewritten modules (old path ↔ new path/functionality).
- **Invariant register:** subsystem invariants are documented, versioned, and review-approved.
- **Determinism checks:** relevant scheduler/IPC/capability behavior remains deterministic where expected.
- **Review closure:** mandatory governance reviewers have approved affected subsystem changes.
- **Handoff package:** documentation bundle includes assumptions, known gaps, and deferred proof-impact items.

## 6) Governance

Subsystem-sensitive changes require domain reviewer sign-off in addition to normal maintainer review.

### Required Reviewers by Area

- **Kernel memory model changes**
  - At least **1 reviewer designated for memory-model correctness**.
  - At least **1 maintainer with architecture bring-up experience** on the reference target.
- **Scheduling subsystem changes**
  - At least **1 scheduler domain reviewer**.
  - At least **1 real-time/latency-aware reviewer** where scheduling semantics are impacted.
- **Capability subsystem changes**
  - At least **1 capability-system reviewer**.
  - At least **1 security-focused reviewer** for authority propagation/revocation impacts.

### Governance Rules

- No self-approval for PRs touching memory model, scheduling, or capability core logic.
- Cross-subsystem changes must receive approval from all impacted subsystem reviewer sets.
- Any exception to this charter requires an explicit waiver documented in the PR and approved by subsystem owners.

---

**Version:** Phase 1 Draft  
**Applies to:** Rust rewrite planning and implementation under this repository for the initial reference platform.
