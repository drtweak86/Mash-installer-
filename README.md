![Banner of the bard](docs/assets/banner_final.png)
# MASH Installer
> **Mythic Assembly & Sigil Heuristics** — forged where neon rain meets stone and every glyph is a command rune.

## What Does It Do?

**MASH Installer is your 3 AM rage-quit recovery tool.** When you've borked your dev station beyond recognition and need it back **NOW**, this is your lifeline.

### The One-Liner (For When You're Raging at 3 AM)

```bash
curl -fsSL https://raw.githubusercontent.com/drtweak86/Mash-installer/work/install.sh | bash
```

`install.sh` in this repo does the heavy lifting: it detects your architecture, grabs the matching release binary (no tarballs, no firmware guesswork), and immediately executes it. Copy, paste, and you vault into the neon forge without thinking twice.

### What You Get

- **System packages** - All the essentials your machine needs
- **Rust toolchain** - Latest stable with cargo tools
- **Git & GitHub CLI** - Version control ready to roll
- **Docker Engine** - Containers at your fingertips
- **Shell & UX** - zsh, starship, and all the goodies
- **Fonts & Themes** - Because even rage-quit terminals deserve to be pretty
- **Buildroot dependencies** - For when you need to compile the world

## 📜 Tavern Card
Step under the warm eaves of the Forge Tavern, leave your frostbitten boots by the hearth, and let the drunk dwarf bard guide you through `MASH-installer`. This tale braids the grit of a thousand dwarves hauling plasma ore, the ferocious snark of a dragon who drinks coffee, and the terse logic of a Ratatui terminal into a neon hymn that knows there's no place like 127.0.0.1. Every log entry is a page from a codex written in neon runes, every dependency a rune etched in starlight, and the newest ritual is a single curl to GitHub Releases followed by a brief extraction:

```bash
CURR_ARCH=$(uname -m)
curl -fsSL https://github.com/drtweak86/Mash-installer/releases/latest/download/mash-setup-${CURR_ARCH}.tar.gz | tar zx
./mash-setup
```

The `mash-setup` binary now embodies the Mythic Assembly & Sigil Heuristics creed: pragmatic Rust craftspersonry, Ratatui glyphs flickering like tavern lanterns, and dry-run gates that keep the neon forge honest.

## ⚙️ What the Bard Recommends
- **First dram:** Run `./mash-setup --help` to see the full CLI menu; the Ratatui UI sings when you pass `--profile dev` or `--profile full`.  
- **Dry-run mode:** Add `--dry-run` to preview every stage; the bard records the plan in `docs/HISTORY.md`.  
- **Green oath:** Always `cd /work/Mash-installer` before running `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`; the forge only crowns green builds.  
- **Commits:** Keep them tight (KCS), back up your work (ABB), test everything (ABT), and ink the changes in `docs/mining-projects/maps*.md` plus this very README (ABD).

## 👑 The Lore of MASH
`Mythic Assembly & Sigil Heuristics` is the championed interpretation, but if the tavern prefers other toasts, consider:

1. **Metal Arcane Systems Hub** — for nights when the neon guts feel industrial.
2. **Missions Above Steel Horizons** — for journeys where the rails are alive with plasma.

No matter the moniker, the creed remains: secure SSH for GitHub, sober Ratatui artistry, and a bard who keeps the story alive in `docs/HISTORY.md`.

## 🧭 Next Steps
When the hearth is warm and Phase 2 is sealed, the bard says: keep the ledger polished, keep the toolchain preheated (`rustfmt`, `clippy`, `sccache`), and let Phase 3 (Pi 4B HDD tuning) sit in the wings until the lane is formally lit.
