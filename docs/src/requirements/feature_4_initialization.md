# Feature 4: Game Initialization & Setup

## User Story 4.1: Start a New Game
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

## User Story 4.2: Create Player Decks
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

## User Story 4.3: View Game Setup in TUI
**As a** player  
**I want** to see the game board and initial state in the TUI  
**So that** I can understand the starting layout of the match.

**TUI Requirements:**
- [ ] Display a main menu with options to "Start Game" and "Quit".
- [ ] On game start, render the main game screen.
- [ ] Display both players' starting health.
- [ ] Render each player's hand, showing the back of the cards for the opponent and the front for the player.
- [ ] Display the number of cards in each player's deck and graveyard.
- [ ] Clearly indicate which player is taking the first turn.
