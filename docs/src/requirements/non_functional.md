# Non-Functional Requirements

## Performance
- [ ] Handle 100+ cards in game state efficiently
- [ ] Blueprint loading completes in < 100ms
- [ ] Turn execution completes in < 1s
- [ ] Memory footprint < 50MB for full game

## Reliability
- [ ] All game state transitions are atomic
- [ ] Invalid game states are impossible to reach
- [ ] Card ID conflicts are detected and prevented
- [ ] Deck validation prevents impossible compositions

## Code Quality
- [ ] All public APIs have documentation
- [ ] Unit tests for core game logic (60%+ coverage)
- [ ] Modular architecture (easily add new card types)
- [ ] Clear separation: Domain logic vs. State management vs. UI

## Extensibility
- [ ] Support custom card abilities (pluggable keyword system)
- [ ] Easy to add new domains/resources
- [ ] Support for future expansion sets
- [ ] Ability to add new game phases without refactoring
