# Feature 2: Card Zones & State Management

## User Story 2.1: Manage Card Zones
**As a** player  
**I want** cards to move between different zones  
**So that** game state accurately reflects card positions

**Requirements:**
- [ ] **Hand**: Cards waiting to be played (max configurable)
- [ ] **Deck**: Cards not yet drawn (supports draw mechanics)
- [ ] **Battlefield**: Active creatures in play (max 9 slots)
- [ ] **Graveyard**: Destroyed or discarded cards
- [ ] Cards maintain unique IDs across zone transitions
- [ ] Query cards by zone efficiently

## User Story 2.2: Draw Cards from Deck
**As a** player  
**I want** to draw cards from my deck each turn  
**So that** I have fresh options to play

**Requirements:**
- [ ] Draw 1 card at start of turn
- [ ] Move card from deck to hand
- [ ] Handle deck mill (no cards left):
  - [ ] Implement mill damage (1 damage per card drawn when deck empty)
  - [ ] Optionally: Player loses game if milled
- [ ] Maximum hand size enforcement (suggest 10 cards)
- [ ] Discard excess cards if hand size exceeded

## User Story 2.3: Manage Card Instances
**As a** developer  
**I want** to track individual card instances separately from blueprints  
**So that** cards maintain unique state and identity

**Requirements:**
- [ ] Each card instance has unique ID
- [ ] Card instances track current health/state
- [ ] Card instances link to blueprint for immutable properties
- [ ] Support querying cards by ID, zone, or type
- [ ] Efficiently manage large numbers of cards

---

## User Story 2.4: Visualize Card Zones in TUI
**As a** player  
**I want** to see all game zones clearly in the TUI  
**So that** I have a complete overview of the game state.

**TUI Requirements:**
- [ ] Render distinct, clearly labeled panels for the player's and opponent's battlefield.
- [ ] The player's hand should be visible and selectable.
- [ ] The opponent's hand should be visible as a count of cards.
- [ ] Display the number of cards remaining in each player's deck.
- [ ] Display the number of cards in each player's graveyard.
- [ ] Implement a way to view the contents of the graveyard (e.g., by selecting it and opening a new list view).
- [ ] The TUI must update all relevant zone displays whenever a card moves (e.g., from Deck to Hand, Hand to Battlefield, Battlefield to Graveyard).
