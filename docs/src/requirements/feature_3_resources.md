# Feature 3: Resource Management

## User Story 3.1: Manage Player Resources
**As a** player  
**I want** to generate and spend resources to play cards  
**So that** resource management becomes a strategic element

**Requirements:**
- [ ] Each player generates resources per turn (mana pool)
- [ ] Resources have multiple types (Neutral, Forest, Volcano, etc.)
- [ ] Resources refresh at start of turn
- [ ] Resources are consumed when playing cards
- [ ] Display available resources to player
- [ ] Prevent playing cards without sufficient resources

## User Story 3.2: Track Domain Resources
**As a** player  
**I want** domain-specific resources to be tracked separately  
**So that** cards have meaningful mana costs

**Requirements:**
- [ ] Support multiple resource domains (Forest, Volcano)
- [ ] Cards specify resource requirements by domain
- [ ] Neutral resources count toward any cost
- [ ] Validate both total and domain-specific resources before play
- [ ] Visual feedback for available resources

---

## User Story 3.3: View Resources in TUI
**As a** player  
**I want** to clearly see my available resources in the TUI  
**So that** I can make informed decisions about playing cards.

**TUI Requirements:**
- [ ] Display the current player's resources in a dedicated, always-visible panel.
- [ ] The resource display should update immediately when resources are generated or spent.
- [ ] Use colors or symbols to differentiate between various resource domains (e.g., green for Forest, red for Volcano, grey for Neutral).
- [ ] The display should be clear and easy to read at a glance.
