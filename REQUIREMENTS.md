# TCG Project - User Stories & Requirements

## Epic 1: Game Initialization & Setup

### User Story 1.1: Start a New Game
**As a** player  
**I want** to initialize a new game with two players  
**So that** we can begin playing a match

**Requirements:**
- [ x ] Load card blueprints from RON files (Forest and Volcano sets)
- [ x ] Initialize two players with starting values:
  - [ x ] Health: 50 HP
  - [ x ] Draw 5 cards into hand from deck
  - [ x ] Empty battlefield
  - [ x ] Empty graveyard
- [ x ] Set game state to active
- [ x ] Determine starting player (player 1 or random)

### User Story 1.2: Create Player Decks
**As a** player  
**I want** to build a deck from available cards  
**So that** I can customize my playstyle

**Requirements:**
- [ ] Support minimum deck size of 30 cards
- [ ] Support maximum deck size of 60 cards
- [ ] Validate deck composition rules
- [ ] Store deck configuration with player profile
- [ ] Support multiple pre-built decks (Forest, Volcano)

---

## Epic 2: Turn System & Player Actions

### User Story 2.1: Execute Turn Sequence
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

### User Story 2.2: Play Cards from Hand
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

## Epic 3: Combat System

### User Story 3.1: Attack with Creatures
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

### User Story 3.2: Block Attacks with Creatures
**As a** player  
**I want** to block incoming attacks with my creatures  
**So that** I can prevent or reduce damage to my health

**Requirements:**
- [ ] Available creatures can be assigned to block
- [ ] Blocking creature deals damage to attacking creature
- [ ] Both creatures may be destroyed depending on power values
- [ ] Update creature health values
- [ ] Move destroyed creatures to graveyard

### User Story 3.3: Apply Card Keywords
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

## Epic 4: Resource Management

### User Story 4.1: Manage Player Resources
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

### User Story 4.2: Track Domain Resources
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

## Epic 5: Card Zones & State Management

### User Story 5.1: Manage Card Zones
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

### User Story 5.2: Draw Cards from Deck
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

### User Story 5.3: Manage Card Instances
**As a** game engine  
**I want** to track individual card instances separately from blueprints  
**So that** cards maintain unique state and identity

**Requirements:**
- [ ] Each card instance has unique ID
- [ ] Card instances track current health/state
- [ ] Card instances link to blueprint for immutable properties
- [ ] Support querying cards by ID, zone, or type
- [ ] Efficiently manage large numbers of cards

---

## Epic 6: Game Ending & Win Conditions

### User Story 6.1: Determine Game Winner
**As a** player  
**I want** to know when the game ends and who wins  
**So that** matches have clear outcomes

**Requirements:**
- [ ] Win condition: Reduce opponent health to 0 or below
- [ ] Win condition: Opponent decks out (mills to 0 cards)
- [ ] Game state transitions to "Ended" when win condition met
- [ ] Display winner and ending reason
- [ ] Prevent further actions after game ends
- [ ] Support concession/surrender

---

## Epic 7: Card Blueprint System

### User Story 7.1: Load Card Blueprints
**As a** game designer  
**I want** to define cards in RON files  
**So that** I can easily add new cards without code changes

**Requirements:**
- [ ] RON files define card properties:
  - [ ] ID, Name, Description
  - [ ] Mana cost (neutral + domain-specific)
  - [ ] Card type (Creature, Spell, Artifact)
  - [ ] For creatures: Power, Health
  - [ ] Keywords and abilities
  - [ ] Rarity (Common, Uncommon, Rare, Mythic)
  - [ ] Domain (Forest, Volcano, etc.)
- [ ] Load blueprints at game startup
- [ ] Cache blueprints in memory
- [ ] Support multiple sets (Forest, Volcanic)
- [ ] Validate blueprint structure

### User Story 7.2: Create Card Variants
**As a** game designer  
**I want** to define multiple card rarities  
**So that** card pool diversity is supported

**Requirements:**
- [ ] Each card can have multiple rarity versions
- [ ] Different rarities may have different stats/costs
- [ ] Deck building respects rarity limits if applicable
- [ ] Blueprints store rarity information

---

## Epic 8: Game State Persistence (Future)

### User Story 8.1: Save Game State
**As a** player  
**I want** to save my game progress  
**So that** I can resume later

**Requirements:**
- [ ] Serialize complete game state
- [ ] Save player boards, hands, decks
- [ ] Save game log of moves
- [ ] Load game from save file
- [ ] Validate save file integrity

---

## Non-Functional Requirements

### Performance
- [ ] Handle 100+ cards in game state efficiently
- [ ] Blueprint loading completes in < 100ms
- [ ] Turn execution completes in < 1s
- [ ] Memory footprint < 50MB for full game

### Reliability
- [ ] All game state transitions are atomic
- [ ] Invalid game states are impossible to reach
- [ ] Card ID conflicts are detected and prevented
- [ ] Deck validation prevents impossible compositions

### Code Quality
- [ ] All public APIs have documentation
- [ ] Unit tests for core game logic (60%+ coverage)
- [ ] Modular architecture (easily add new card types)
- [ ] Clear separation: Domain logic vs. State management vs. UI

### Extensibility
- [ ] Support custom card abilities (pluggable keyword system)
- [ ] Easy to add new domains/resources
- [ ] Support for future expansion sets
- [ ] Ability to add new game phases without refactoring
