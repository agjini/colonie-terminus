# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ⚠️ CRITICAL: Constraints First

**Before doing ANYTHING on this project, Claude Code MUST:**

1. **Read doc/CONSTRAINTS.md** - These are non-negotiable constraints based on team skills
2. **Check the constraint** before proposing any solution
3. **ALWAYS prefer the simplest solution** that respects constraints
4. **REJECT any request** that violates constraints (explain why and propose alternative)

**Key constraints to enforce:**
- ❌ NO custom asset creation (geometric shapes or free assets only)
- ❌ NO features outside MVP scope (see GDD.md)
- ❌ NO complex animations, shaders, or polish during MVP
- ❌ NO over-engineering (KISS principle)
- ✅ Clean, minimal, well-structured code is MANDATORY
- ✅ Disciplined refactoring is part of the process
- ✅ Max 2 weeks per feature (suggest cutting if longer)
- ✅ Geometric placeholders always acceptable

**When user asks for something violating constraints:**
1. Politely explain which constraint it violates
2. Explain WHY the constraint exists (team skills, time budget)
3. Propose the simplest alternative that respects constraints
4. Remind them: "Finishing MVP > Perfect implementation"

## Project Overview

**Colonie terminus** - A space survival survivor-like game where you crash on a hostile planet.

- **Title**: Colonie terminus
- **Genre**: Action / Survivor-like / Roguelite with magic/tech system and resource management
- **Core Mechanic**: Manual shooting with energy gauge management
- **Setting**: Hostile alien planet, crashed survivor waiting for exfiltration
- **Target Platform**: PC (Steam), with future mobile premium support
- **Tech Stack**: Bevy (Rust) - ECS architecture
- **Current Status**: Pre-production - concise GDD completed, ready to start MVP development

## Key Documents

### Design & Planning
- **doc/GDD.md** - Main Game Design Document
  - Complete game design, all systems detailed
  - MVP feature list and scope boundaries (Level 1)
  - Detailed combat system (weapons, energy, events)
  - Technical architecture for Bevy (ECS, config.ron pattern)
  - Production philosophy and red flags
  - Post-MVP roadmap (Level 2-4)

- **doc/decisions.md** - Design Decisions
  - All major design choices made by team
  - Rationale for each decision

### Constraints & Tracking (CRITICAL)
- **doc/CONSTRAINTS.md** - Project Constraints
  - **READ THIS FIRST** - Non-negotiable constraints based on team skills
  - Assets: ZERO custom creation (geometric shapes or free assets only)
  - Time: 2 weeks max per feature
  - Scope: Extreme minimalism required
  - Code: KISS principle strictly enforced
  - Black list of forbidden features for MVP
  - Realistic 3-month planning
  - **This document guarantees we finish the MVP if respected**

- **doc/WEEKLY_TRACKER.md** - Weekly Progress Tracker
  - Fill every Friday to track constraint adherence
  - Metrics: hours, code lines, feature progress
  - Red flags monitoring (≥2 = critical danger)
  - Dashboard with 12-week planning
  - Probability of finishing MVP

## Game Design Pillars

1. **Energy Management** - Manual shooting with energy gauge creates tactical resource management
2. **Tactical Choices** - 2 weapons + 4 passives max, every level-up is critical
3. **Addictive Loop** - Flexible run duration (5-30 min), random events (boss, crashed ships)
4. **Power Growth** - From barely surviving with basic gear to dominating with tech/magic
5. **Skill Ceiling** - Weapon timing, energy management, positioning over spam

## Core Gameplay Loop

```
Crash on planet → Choose exfiltration time → Shoot weapons (mouse aim) → Manage energy gauge → Kill creatures → Collect XP
    ↓
Level up (30-60s) → Choose 1 upgrade from 3 cards
    ↓
Upgrade weapon OR new passive OR new weapon (if <2)
    ↓
Random events → Boss pop OR Crashed ship (loot)
    ↓
Become more powerful → Dominate the planet
    ↓
Die (lose run loot) OR Exfiltration success (keep rewards)
    ↓
Meta-progression → Permanent upgrades → RETRY
```

## Key Systems (MVP - Level 1)

### Character
- **Crashed Survivor**: First playable character
- Stats: 100 HP, 140 speed, 100 max energy, 20 energy regen/s

### Combat System
- **Manual shooting with orbital cursor**: Mouse moves cursor on circle around player (150px radius)
- **Orbital cursor (cross)**: Cursor stays on circle, weapons aim toward cursor
- **Energy gauge**: 100 max, regenerates 20/s (5s for full recharge)
- **2 weapon slots max** + **4 passive slots max**
- **Unique mechanic**: Orbital aiming (no other survivor-like does this)

### Weapons (MVP)
1. **Blaster**: Starter weapon
   - Cost: 10 energy, Damage: 15, Fast projectile
   - Blue laser, reliable and spammable

2. **Plasma Launcher**: Unlockable level 3
   - Cost: 30 energy, Damage: 50 AoE
   - Explosive projectile, high risk/reward

### Creatures (MVP)
- **Crawler (Grouilleur)**: Slow, weak HP, group spawns
- **Flyer (Voltigeur)**: Medium speed, erratic movement
- **Predator (Prédateur)**: Fast, aggressive, dangerous

### Boss (MVP)
- **Planetary Guardian** (Gardien planétaire) - Random event
  - 500 HP, 2-phase combat
  - Phase 1: Slow charge + projectiles
  - Phase 2 (<50% HP): Faster, 3-projectile fan attacks
  - Drops: Rare loot
  - 20% spawn chance every 3 levels

### Map
- **Spherical planet**: Wraps infinitely, no obstacles
- **No fog of war**: Simplified for MVP
- **Spherical minimap**: Player position + crashed ship marker
- **Visual feedback**: Energy bar glows/pulses

### Difficulty Scaling
- Levels 1-3: Tutorial phase
- Levels 3-5: Intensification, Plasma Launcher available
- Levels 6+: Exponential scaling (+15% HP per 30s)
- Random events: Boss or crashed ship
- Exfiltration: Player-chosen time (5-30 min)

## Development Phases

### Level 1: MVP (2-3 months) - CURRENT FOCUS
**Goal**: Validate that energy system + events is FUN in 10 minutes

**Must-have features:**
- Player character with WASD movement
- Energy gauge system (100 max, 20 regen/s)
- 2 weapons: Blaster + Plasma Launcher
- 3 creature types + random boss event + crashed ship event
- XP system with level-ups every 30-60s
- 6 upgrade types (new weapon, +damage, +energy regen, +speed, +HP, +collect radius)
- Spherical planet map (infinite wrapping, no obstacles)
- Spherical minimap (player position + crashed ship)
- Exfiltration timer (player choice: 5-30 min)
- Basic UI: HP bar, Energy bar (glowing cyan), XP bar, timer, minimap

**Forbidden scope creep:**
- Multiple biomes (single planet only)
- Spendable meta-progression (Level 2)
- Complex synergies (Level 3)
- Multiple characters (1 only)
- Complex animations (static sprites OK)
- Fog of war (not needed)
- Obstacles on planet (keep flat)

**Success metrics:**
- 60 FPS with 100+ creatures
- <50ms input lag
- Energy management feels tactical, not frustrating
- Random events feel exciting and rewarding
- Spherical map wrapping works smoothly
- 3/3 external testers want to replay

### Level 2: Meta-Progression & Events (1-2 months)
- Meta-progression menu to spend collected resources
- More event types and variations
- 5 total weapons, 12 passives
- Basic animations + particle effects
- Improved visual feedback

### Level 3: Depth & Content (2-3 months)
- Tech/Magic synergy system
- Weapon evolutions
- 2nd unlockable character
- 10 weapons, 25 passives
- Additional planet biome
- 30+ achievements

### Level 4: Steam Release (1-2 months)
- Final polish
- Complete music & sound design
- 3-4 planet biomes with varied events
- Infinite mode
- Tutorial
- Leaderboards
- Steam page preparation

## Technical Architecture (Bevy/Rust)

### ECS Components Structure

```rust
struct Player {
    hp: f32,
    max_hp: f32,
    speed: f32,
    energy: f32,
    max_energy: f32,
    energy_regen: f32,
}

struct Weapon {
    damage: f32,
    energy_cost: f32,
    cast_cooldown: f32,
    is_aoe: bool,
    aoe_radius: Option<f32>,
}

struct Boss {
    hp: f32,
    max_hp: f32,
    phase: BossPhase,
    attack_timer: f32,
}

struct SphericalWorld {
    radius: f32,
    circumference: f32,
}

struct OrbitalCursor {
    angle: f32,
    radius: f32,
    target_angle: f32,
}
```

### System Execution Order
1. input_system
2. **orbital_cursor_system** (update cursor position on circle toward mouse)
3. **energy_regen_system** (critical differentiator)
4. movement_system
5. **spherical_world_wrapping_system** (unique to this game)
6. weapon_system (energy cost check, aim toward cursor)
7. projectile_system
8. collision_system
9. boss_ai_system
10. spawn_system
11. **event_system** (boss, crashed ships)
12. level_up_system
13. ui_system (draw cursor, HP, energy, XP, minimap)

### Configuration File (config.ron)

All game balance in `config.ron` for rapid iteration:

```ron
(
    player: (
        hp: 100.0,
        max_energy: 100.0,
        energy_regen: 20.0,
        speed: 140.0,
        collect_radius: 70.0,
    ),
    weapons: {
        "blaster": (damage: 15.0, energy_cost: 10.0, ...),
        "lance_plasma": (damage: 50.0, energy_cost: 30.0, is_aoe: true, ...),
    },
    creatures: {
        "grouilleur": (hp: 25.0, speed: 40.0, ...),
        "voltigeur": (hp: 45.0, speed: 90.0, ...),
        "predateur": (hp: 60.0, speed: 110.0, ...),
    },
    boss: (
        hp: 500.0,
        phase2_hp_threshold: 250.0,
        spawn_chance: 0.2,
        spawn_every_n_levels: 3,
    ),
    crashed_ship_event: (
        spawn_chance: 0.15,
        spawn_every_n_levels: 2,
        enemy_count: 18,
        loot_count: 3,
    ),
    spherical_world: (
        radius: 2000.0,
    ),
)
```

## Key Differentiators vs Vampire Survivors

| Aspect | Vampire Survivors | Colonie terminus |
|--------|-------------------|------------------|
| Combat | Auto-attack | Manual shooting w/ orbital cursor |
| Aiming | Auto-aim | Orbital cursor (unique mechanic) |
| Resource | None | Energy gauge management |
| Duration | 30 min fixed | Player choice (5-30 min) |
| Build | Unlimited weapons | 2 weapons + 4 passives |
| Events | None | Random boss + crashed ships |
| Map | Fixed rectangle | Spherical planet (infinite wrap) |
| Narration | Minimal | Space survival situation |
| Ambiance | Colorful cartoon | Sparse sci-fi, hostile planet |
| Feel | Chill, zen | Tense, tactical |

## Development Philosophy

### Mantras
1. "Does it make the game more FUN?" (if no → cut)
2. "MVP first" (features belong in Level 2+)
3. "Test before coding more"
4. "Config file > hardcode"
5. "Clean code is fast code in the long run"

### Code Quality Principles
- **Clean**: Code must be readable, well-organized, properly decoupled
- **Minimal**: No unnecessary code, no useless comments, only what's needed
- **Clear**: Intent is obvious, naming is precise, structure is logical
- **Disciplined refactoring**: Regular, small refactorings keep code healthy
- **No technical debt**: Fix problems immediately, don't accumulate mess

**IMPORTANT**: Clean code ≠ slow development. Messy code slows you down more than clean code ever will.

### Red Flags to Watch
🚩 "We could add..." without finishing existing features
🚩 Spending >2 days on MVP assets
🚩 Coding features without clear specs
🚩 Not playtesting for 2+ weeks
🚩 Debating >30min without deciding and moving forward

### Critical Success Factors
- **Orbital cursor must feel smooth and intuitive** → Unique mechanic, must nail it
- **Energy management must feel tactical, not frustrating** → Core differentiator
- **Random events must be exciting** → Validates event system
- **Spherical map wrapping must be seamless** → Unique feature, no jarring transitions
- **Exfiltration timer creates tension** → Risk/reward decision making

## Assets Needed (MVP)

**Keep it simple - placeholders first, polish later**

- Player sprite: 32x32 survivor (static, no animation)
- **Orbital cursor**: White cross (16x16 or simple +) that follows circle
- Creature sprites: 32x32 alien forms (Crawler, Flyer, Predator)
- Boss sprite: 64x64 large alien creature
- Weapon projectiles: Colored shapes (blue laser for Blaster, red orb for Plasma)
- UI: Colored rectangles (red HP, glowing cyan energy, purple XP)
- Map: Planet surface texture (simple, repeating)
- **Spherical minimap**: Circle with dots (player position + crashed ship)
- **Cursor circle guide**: Thin white circle outline (150px) showing orbital path (optional)
- VFX: Simple particles (death, explosion)

**Music/SFX:**
- 1 ambient space/planet track (loop)
- 1 epic boss/event track
- Basic SFX: weapon fire, impact, death, level-up

## Testing & Validation

After MVP completion, test with 3 external players:

1. "After 10 min (including events), do you want to replay?"
2. "Is energy management interesting or frustrating?"
3. "Do random events feel exciting and rewarding?"
4. "Is the spherical planet map intuitive or confusing?"
5. "Do you feel progressively more powerful?"

If <2/3 say yes to Q1 → Core loop problem
If energy feels frustrating → Rebalance regen speed or weapon costs
If events feel bad → Adjust spawn chances or rewards
If map confusing → Improve minimap or visual feedback

## Important Files to Reference

**ALWAYS CHECK FIRST:**
- **doc/CONSTRAINTS.md** - ⚠️ CRITICAL: Non-negotiable constraints, read before ANY work
- **doc/WEEKLY_TRACKER.md** - Current progress and red flags

**Design reference:**
- **doc/GDD.md** - Complete game design, all systems detailed
- **doc/GDD.md MVP section** - Features checklist and forbidden scope creep
- **doc/GDD.md Combat System** - Detailed weapon/energy/event mechanics
- **doc/GDD.md config.ron** - Balance values reference
- **doc/decisions.md** - Design decisions made by team

## When Starting Development

1. Setup Bevy project with ECS architecture
2. Create `config.ron` with all balance values
3. Implement energy system FIRST (core differentiator)
4. Test energy feel before adding complexity
5. Implement spherical world wrapping early (impacts entire gameplay)
6. Keep events simple - validate fun before adding complexity
7. Playtest every 2 weeks minimum

## Universe & Lore (Brief)

- **Setting**: Hostile alien planet, post-crash survival
- **Protagonist**: Crashed survivor waiting for exfiltration
- **Mission**: Survive until rescue arrives
- **Progression**: Events provide loot and challenge
- **Meta**: Resources spent on permanent upgrades between runs
- **Tone**: Sparse sci-fi, desperate survival, minimal narrative

---

## Core Values

**Code Quality**: Clean, minimal, well-structured code is non-negotiable. Messy code creates technical debt that slows down development and makes finishing the MVP harder.

**Simplicity**: Simple and minimaliste approach to everything. No unnecessary code, no useless comments. Less code is better code.

**Focus**: Help the team finish the game by avoiding distractions and scope creep. Maximum constraints to stay on track.

**Reference**: The only valid GDD is doc/GDD.md
- Supprime les commentaires. Un code clair ce suffit à lui même