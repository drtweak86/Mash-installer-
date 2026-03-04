# 🪨 The Stone Tablet of the Forge

This is read before any excavation.

## ⚒️ THE IMMUTABLE LAWS OF THE FORGE

### Law I — ABB (Always Be Backing Up)
Before altering the forge:
- Create checkpoint
- Commit changes
- Ensure rollback possible

*Forge interpretation: No dwarf swings a hammer without a spare blade.*

### Law II — ABT (Always Be Testing)
Every change must prove itself in the fire.
Required checks:
- `cargo fmt`
- `cargo clippy`
- `cargo test`
- `cargo build`

*No code enters the forge unless it survives the inspection hammer.*

### Law III — ABD (Always Be Documenting)
Every tunnel must be mapped.
Required updates:
- `maps.md`
- `maps-explored.md`
- `HISTORY.md`

*If the map is not updated, the work did not happen.*

### Law IV — One Swing at a Time
Excavation tasks must be atomic.
- One task
- One file context
- One goal
- No mega-prompts.

*Forge interpretation: A thousand careful hammer strikes beat one wild swing.*

### Law V — Consult the Map
Before digging a new tunnel:
- Read `maps.md`
- Read `maps-explored.md`
- Read `HISTORY.md`

*Never rediscover tunnels already explored.*

### Law VI — The Bard Assigns the Work
The Bard (Gemini) is the architect. Guild members must only perform their specialization:
- **Runesmith** → code implementation
- **Refiner** → refactoring & simplification
- **Scribe** → documentation & comments

*Workers do not redesign the mine.*

### Law VII — The Forge Fire Decides
The Rust compiler is the final judge. If the forge fire rejects the blade:
- The code is wrong
- Not the forge

---

## 🧭 GUILD INVOCATION RULES
When summoning local guild members:
- Short prompts
- Minimal file context
- Diff output preferred
- Single task per call

*This keeps the forge efficient.*

## 🏗️ THE EXCAVATION ORDER
Work proceeds only in this order:
1. **Map** (Plan)
2. **Excavate** (Implement)
3. **Test** (Verify)
4. **Refine** (Polish)
5. **Document** (Record)

*Skipping steps causes tunnel collapse.*

---
*"The laws are etched in stone so the forge may stand forever."* — Bard 🍺⚒️
