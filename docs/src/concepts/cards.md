# Cards

Cards are the fundamental building blocks of the game. Each player builds a deck of cards to play with. All cards have a few things in common: a `name`, a `description`, a `domain` (the thematic and magical 'color' of the card), a `rarity`, and a `cost`.

## Card Types

There are four main types of cards in the game.

### 1. Creatures

Creatures are the primary way to attack your opponent and defend yourself. They have `power` and `health` values.

- **Power**: The amount of damage a creature deals in combat.
- **Health**: The amount of damage a creature can sustain before it is defeated and sent to the graveyard.

### 2. Lands

Lands are the source of magical resources in the game. They do not typically have a casting cost. Once per turn, a player can play a Land card from their hand.

Each Land card specifies which `resource` (or "mana") it produces and the `amount`. For example, a "Volcanic Vent" produces 1 `Volcano` resource.

### 3. Spells

Spells have a one-time effect when they are played. The specific effect is detailed in the card's `action`. Examples of spell actions include:

- **Damage**: Deals a certain amount of damage to a target (e.g., a creature or player).
- **Heal**: Restores health to a target.
- **ModifyStats**: Temporarily or permanently changes a creature's power and/or health.
- **GrantKeyword**: Gives a creature a new keyword for a specific duration.

Spells can be "slow" (Sorcery speed) or "fast" (Instant speed). A spell with the **Quick** keyword can be played at any time, even during your opponent's turn. Spells without this keyword can only be played during your own main turn phase.

### 4. Artifacts

Artifacts are persistent cards that remain on the battlefield and provide a continuous effect or an activated ability.

- **Static Artifacts**: These provide a passive effect that is always active while the artifact is on the battlefield. For example, an artifact might give all of your creatures +1 power.
- **Activated Artifacts**: These have an ability that you can use by paying a specified resource cost. For example, an artifact might allow you to pay 2 mana to heal a creature.

## Keywords

Keywords are shorthand for common abilities that cards can have.

- **Intercept**: This creature can choose to become the target of an attack directed at an adjacent allied creature.
- **Stealth**: This creature cannot be targeted by attacks or spells for a certain duration.
- **Crush**: When this creature attacks, any excess damage beyond the defending creature's health is dealt to the opponent.
- **Aerial**: This creature can only be blocked by other creatures with Aerial.
- **Quick**: This spell or ability can be played at "Instant" speed, meaning you can play it at any time, including during your opponent's turn.
