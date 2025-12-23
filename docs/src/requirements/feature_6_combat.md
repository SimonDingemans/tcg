# Feature 6: Combat System

## User Story 6.1: Attack with Creatures
**As a** player  
**I want** to attack the opponent with my creatures  
**So that** I can deal damage and reduce their health

**Requirements:**
- [ ] Only creatures on battlefield can attack
- [ ] Creatures must have been on battlefield since before current turn (summoning sickness)
- [ ] Select target (opponent or opposing creature)
- [ ] Calculate damage (creature power attribute)
- [ ] Apply damage to target
- [ ] Update creature state after attack (tapped/exhausted)
- [ ] Remove destroyed creatures (health ≤ 0) to graveyard

## User Story 6.2: Block Attacks with Creatures
**As a** player  
**I want** to block incoming attacks with my creatures  
**So that** I can prevent or reduce damage to my health

**Requirements:**
- [ ] Available creatures can be assigned to block
- [ ] Blocking creature deals damage to attacking creature
- [ ] Both creatures may be destroyed depending on power values
- [ ] Update creature health values
- [ ] Move destroyed creatures to graveyard

## User Story 6.3: Apply Card Keywords
**As a** player  
**I want** cards with special abilities to trigger correctly  
**So that** strategic card choices matter

**Requirements:**
- [ ] **Intercept**: Creatures can block even if not normally able
- [ ] **Stealth**: Creatures cannot be blocked (unblockable)
- [ ] **Flying**: Creatures can only be blocked by flying creatures
- [ ] Keyword effects execute at correct game phase
- [ ] Keyword interactions stack correctly

---

## User Story 6.4: Execute Combat in TUI
**As a** player  
**I want** to perform combat actions through the TUI  
**So that** I can attack my opponent and defend against their attacks.

**TUI Requirements:**
- [ ] Implement a "Combat Phase" mode in the TUI, activated by a key press (e.g., 'a' for Attack).
- [ ] Allow selection of creatures on the player's battlefield to be marked as attackers.
- [ ] Once attackers are chosen, allow the player to select a target for each attacker (the opponent or a specific enemy creature).
- [ ] For the defending player, allow them to select their available creatures and assign them as blockers to specific attackers.
- [ ] The TUI must clearly show which creatures are attacking, which are blocking, and what the expected outcomes are.
- [ ] After combat is confirmed, update the TUI to reflect new health totals, tapped/exhausted states, and move any destroyed creatures to the graveyard.
- [ ] Display card keywords (like Flying, Stealth) in the card detail panel to inform combat decisions.
