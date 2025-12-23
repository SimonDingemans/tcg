# TCG TUI Development Plan

This document outlines the steps to build a Terminal User Interface (TUI) to mitigate the business risk of the project lacking a user-facing application.

## Step 1: Incorporate UI Libraries

Begin by integrating specialized libraries for terminal rendering and user input into the project's dependency management system. These libraries provide the foundational tools for creating layouts, interface components, and handling user interactions, forming the building blocks of the TUI.

## Step 2: Isolate UI Code

To ensure a clean architecture, create a dedicated and separate module within the source code to house all TUI-related logic. This isolates the user interface from the core game engine. The application's main entry point must be updated to initialize and launch this new UI module, making it the primary interface for the user.

## Step 3: Develop the Core UI Loop

Implement a central, continuous loop that serves as the heart of the interactive experience. On each iteration, this loop will be responsible for three key actions:
1.  Drawing all visual components to the screen.
2.  Polling for and processing any user input.
3.  Updating the TUI's internal state, such as tracking the currently selected item.

The application will exit this loop only when the user chooses to quit.

## Step 4: Design the Visual Layout

Define the logical structure of the user interface. This involves strategically dividing the screen space into distinct areas to display all relevant game information. Key areas should include:
*   An area for the current player's hand, battlefield, and resources.
*   A similar area for the opponent's visible game elements.
*   A scrollable log to display the history of game events.
*   An information panel to show details of a selected card or element.
*   A status bar to show available commands and help information.

## Step 5: Connect Game State to Visuals

Establish a clear data flow from the core game engine to the TUI. The UI should act as a read-only visual representation of the game's current state. When the game state changes (e.g., a card is drawn), the UI must automatically update to reflect that change. The TUI will also manage its own separate, view-specific state, such as which element is currently selected by the user, without altering the core game state directly.

## Step 6: Implement User Input Handling

Develop a system to capture and interpret keyboard inputs from the user. These inputs need to be translated into meaningful game actions. For example, using the arrow keys should change the selected card, and pressing a specific key should trigger the "play card" action. These action requests are then sent to the core game engine for validation and processing.

## High-Level Project Flow

1.  **Integrate Dependencies:** Add the required TUI libraries to the project.
2.  **Structure UI Module:** Create an isolated module for all UI-related code.
3.  **Launch from Main:** Update the application's entry point to start the TUI.
4.  **Build Core Loop:** Implement the main loop for drawing, input, and state updates.
5.  **Design Layout:** Define the screen areas for all game components.
6.  **Render Game State:** Implement the logic to visually represent the game data.
7.  **Handle Input:** Implement the system for capturing and translating user commands.
8.  **Connect to Engine:** Funnel user commands to the game engine for processing.
