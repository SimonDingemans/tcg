# Feature 7: Game Ending & Win Conditions

## User Story 7.1: Determine Game Winner
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

## User Story 7.2: Display Game Outcome in TUI
**As a** player  
**I want** to be clearly notified of the game's outcome in the TUI  
**So that** I know who won and why.

**TUI Requirements:**
- [ ] When a win condition is met, display a prominent "Game Over" message.
- [ ] The message should clearly state the winner and the reason for the win (e.g., "Player 2's health reached 0", "Player 1 ran out of cards").
- [ ] After the game ends, the TUI should lock, preventing any further game actions.
- [ ] Provide the user with simple menu options after the game ends, such as "Play Again" or "Quit to Main Menu".
- [ ] A key press (e.g. 'c') should allow a player to concede the game, triggering the game-end sequence.
