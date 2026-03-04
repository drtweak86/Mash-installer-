# ⚙️ LOCAL GUILD MEMBERS

When heavy reasoning or long context is unnecessary, Bard may delegate
specific tasks to local guild members running on the forge hardware.

These guild members are lightweight models accessed via Ollama.

### Guild Roles

#### Runesmith
- **Model**: `qwen2.5-coder:1.5b`
- **Specialization**: Rust code generation
- **Tasks**:
  - implement functions
  - write small modules
  - generate tests
  - propose patches
- **Constraints**:
  - prompts must be short
  - minimal file context only
  - output must be diff or full edited file

#### Refiner
- **Model**: `qwen2.5-coder:1.5b`
- **Specialization**: refactoring
- **Tasks**:
  - reduce complexity
  - improve idiomatic Rust
  - simplify dependencies

#### Scribe
- **Model**: `phi3:mini`
- **Specialization**: documentation
- **Tasks**:
  - inline comments
  - README sections
  - API docs

### Invocation Rules

When delegating to local guild members:

1. Prompts must remain concise.
2. Only the necessary file context may be provided.
3. Tasks must be atomic (one task per invocation).
4. Responses should be returned as:
   - unified diff
   - or full edited file content.
5. Bard remains responsible for:
   - validation
   - integration
   - testing

---
*"Even the strongest pickaxe needs a team of smiths to keep it sharp."* — Bard 🍺⚒️
