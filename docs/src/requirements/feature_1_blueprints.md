# Feature 1: Card Blueprint System

## User Story 1.1: Load Card Blueprints
**As a** game designer  
**I want** to define cards in RON files  
**So that** I can easily add new cards without code changes

**Requirements:**
- [ ] RON files define card properties:
  - [ x ] ID, Name, Description
  - [ x ] Mana cost (neutral + domain-specific)
  - [ x ] Card type (Creature, Spell, Artifact)
  - [ x ] For creatures: Power, Health
  - [ x ] Keywords and abilities
  - [ x ] Rarity (Common, Uncommon, Rare, Epic, Legendary)
  - [ x ] Domain (Forest, Volcano, etc.)
- [ x ] Load blueprints at game startup
- [ x ] Cache blueprints in memory
- [ x ] Support multiple sets (Forest, Volcanic)
- [ x ] Validate blueprint structure

## User Story 1.2: Create Card Variants
**As a** game designer  
**I want** to define multiple card rarities  
**So that** card pool diversity is supported

**Requirements:**
- ~~[ ] Each card can have multiple rarity versions~~
  - Will be done in a later iteration
- ~~[ ] Different rarities may have different stats/costs~~
  - Will be done in a later iteration
- [ x ] Deck building respects rarity limits if applicable
- [ x ] Blueprints store rarity information
