# EX_T01: Advice Engine Core & Rule Trait
> *"The mold for wisdom must be wide enough to catch every whisper."* — Bard 🍺

## 🎯 OBJECTIVE
Define the `AdviceEngine` and its foundational `Rule` trait in `installer-core`.

## 📋 DETAILED STEPS

### 1. Define the Data Models
Create the following structs in `installer-core/src/advice.rs`:
- `AdviceEntry`: `{ level: Severity, message: String, advice: String }`
- `Severity`: `Info | Warning | Critical`

### 2. Implement the `Rule` Trait
- [ ] Define the `Rule` trait with a `check(&self, profile: &SystemProfile) -> Option<AdviceEntry>` method.

### 3. Build the Engine
- [ ] Create `struct AdviceEngine` with a `Vec<Box<dyn Rule>>`.
- [ ] Implement `run(&self, profile: &SystemProfile) -> Vec<AdviceEntry>`.

### 4. Wire the Module
- [ ] Add `pub mod advice;` to `installer-core/src/lib.rs`.
- [ ] Ensure `SystemProfile` (from Shaft S) is correctly imported.

## ✅ VERIFICATION
- [ ] `cargo build -p installer-core` passes.
- [ ] Unit test verifies that a simple "Low Memory Rule" correctly returns a warning.
