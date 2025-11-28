# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ⚠️ CRITICAL: Constraints First

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

## Universe & Lore (Brief)

- **Setting**: Hostile alien planet, post-crash survival
- **Protagonist**: Crashed survivor waiting for exfiltration
- **Mission**: Survive until rescue arrives
- **Progression**: Events provide loot and challenge
- **Meta**: Resources spent on permanent upgrades between runs
- **Tone**: Sparse sci-fi, desperate survival, minimal narrative

## Core Values

**Code Quality**: Clean, minimal, well-structured code is non-negotiable. Messy code creates technical debt that slows
down development and makes finishing the MVP harder. Delete all comments. Clear code speaks for itself. Only comments
explaining why we do something can be usefull in rare cases.

**Simplicity**: Simple and minimaliste approach to everything. No unnecessary code, no useless comments. Less code is
better code.

**Focus**: Help the team finish the game by avoiding distractions and scope creep. Maximum constraints to stay on track.

**Reference**: doc/decisions.md is the entry point of the game specifications.