# Feature 5: Turn System & Player Actions

## User Story 5.1: Execute Turn Sequence
**As a** player  
**I want** to take my turn in a structured sequence  
**So that** gameplay is fair and predictable

**Requirements:**
- [ ] Begin turn phase:
  - [ ] Refresh available resources
  - [ ] Draw one card from deck
- [ ] Main phase:
  - [ ] Play cards from hand (costs permitting)
  - [ ] Attack with creatures on battlefield
  - [ ] Play spell cards
- [ ] End turn phase:
  - [ ] Finalize all pending actions
  - [ ] Pass turn to opponent

## User Story 5.2: Play Cards from Hand
**As a** player  
**I want** to play cards from my hand to the battlefield or as spells  
**So that** I can develop my board state

**Requirements:**
- [ ] Validate player has sufficient resources (mana)
- [ ] Validate target zone has available slots (max 9 creatures)
- [ ] Move card from hand to appropriate zone
- [ ] Deduct resource cost from player pool
- [ ] Update card instance with battlefield status

---

## User Story 5.3: Interact with Turns and Cards in TUI
**As a** player  
**I want** to see whose turn it is and play cards via the TUI  
**So that** I can perform my basic actions.

**TUI Requirements:**
- [ ] Clearly highlight the currently active player.
- [ ] The TUI should update automatically when a card is drawn at the start of a turn.
- [ ] Allow the player to use arrow keys (or similar) to select different cards in their hand.
- [ ] Display details of the currently selected card in a dedicated panel.
- [ ] A specific key press (e.g., 'p' for Play) should trigger the "play card" action for the selected card.
- [ ] The TUI must update to show the card moving from the hand to the battlefield.
- [ ] Provide feedback in a game log if an action is invalid (e.g., "Not enough mana").
- [ ] Implement a key press (e.g., 'e' for End) to end the current turn.
