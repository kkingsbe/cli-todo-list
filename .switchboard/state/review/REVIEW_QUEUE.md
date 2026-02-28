# Code Review Queue

## Sprint 1

---

### story-01-1: Initialize Cargo Project

- **Status:** ❌ CHANGES_REQUESTED
- **Reviewed by:** code-reviewer
- **Review date:** 2026-02-28T17:50:03Z
- **Acceptance Criteria:**
  - [x] Cargo.toml created with all required dependencies — MET
  - [x] src/main.rs created with working entry point — MET
  - [x] Build passes: cargo build --release — MET (exit code 0)
  - [x] Tests pass: cargo test — MET (85 tests pass)
  - [ ] Project compiles without warnings: cargo clippy -- -D warnings — **NOT MET**
- **Must Fix:**
  1. Clippy fails with `-D warnings` - 35 dead code warnings
     - Current: 35 errors from `cargo clippy -- -D warnings` (unused functions, structs, enums)
     - Expected: clippy should pass with exit code 0
     - Why: Story explicitly requires "Project compiles without warnings" as acceptance criterion
- **Should Fix:**
  1. Consider adding `#[allow(dead_code)]` to placeholder stubs if they are intentionally unused
- **Requeue Instructions:** 
  - Fix clippy warnings so `cargo clippy -- -D warnings` passes
  - The issue is that story-01-2 introduced full module implementations that create dead code warnings
  - Either: (a) Add `#[allow(dead_code)]` attributes to unused code, OR (b) Keep modules as minimal stubs until they're needed
  - Post-check: Build + tests pass AND clippy -D warnings passes

---

### story-01-2: Create Module Structure

- **Status:** ❌ CHANGES_REQUESTED
- **Reviewed by:** code-reviewer
- **Review date:** 2026-02-28T17:50:03Z
- **Acceptance Criteria:**
  - [x] All 9 module files created in src/ — MET
  - [x] src/lib.rs declares all modules — MET
  - [x] Build passes: cargo build --release — MET (exit code 0, 36 warnings)
  - [x] Tests pass: cargo test — MET (85 tests pass)
  - [x] Each module has proper doc comments — MET
  - [x] Each module has unit tests — MET
- **Must Fix:**
  1. **Scope violation**: The module files contain full implementations (Task, Tag, Priority, Status, filter structs, error types, etc.) rather than empty placeholder stubs as specified in the story's "Implementation Plan"
     - Current: Full implementations with 35 dead code warnings
     - Expected: Minimal placeholder modules per story scope
     - Why: The story specifies "Create empty module files" with placeholder content, but the implementations exceed scope and cause clippy failures in story-01-1
  2. Clippy warnings cause story-01-1 to fail acceptance criteria
- **Should Fix:**
  1. Each module has significant dead code (unused functions) - consider whether full implementation was intended at this stage
- **Requeue Instructions:**
  - Reduce module implementations to minimal stubs per the original story specification
  - Add `#[allow(dead_code)]` attributes to suppress warnings for intentionally unused placeholder code
  - Ensure `cargo clippy -- -D warnings` passes for story-01-1's acceptance criteria
  - Post-check: Build + tests pass AND clippy -D warnings passes

---

### Previous Review Entries (Completed)

