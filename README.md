# Tower Defense Game Backend

A backend server for a multiplayer tower defense game built with Rust. This project provides the game logic and WebSocket server for real-time gameplay.

## Features

- **Real-time Multiplayer**: WebSocket-based server for low-latency gameplay
- **Game Lobbies**: Create and join game lobbies with unique IDs
- **Chat System**: In-game chat functionality between players
- **Save/Load Games**: Ability to save game state and load it later
- **Dynamic Tower & Enemy Types**: Allows a modular approach to adding new tower and enemy types
- **Tower Upgrade System**: Upgrade towers to more powerful versions
- **Economy System**: Earn coins by defeating enemies to build and upgrade towers
- **Path-based Movement**: Enemies follow predefined paths on the map

## Technologies Used

- **Rust**: Core programming language
- **Tokio**: Asynchronous runtime
- **Warp**: Web server framework
- **WebSockets**: Real-time communication
- **Serde**: Serialization/deserialization for game state
- **JSON**: Data format for client-server communication

## Project Structure

```
tower-defense-backend/
├── Cargo.toml                  # Workspace configuration
├── tower-defense/              # Core game logic
│   ├── src/
│   │   ├── entity/             # Game entities (towers, enemies)
│   │   ├── map/                # Map and path definitions
│   │   ├── math/               # Vector and rectangle utilities
│   │   └── game.rs             # Main game logic
├── tower-defense-server/       # WebSocket server
│   ├── src/
│   │   ├── game/               # Game server and lobby management
│   │   ├── handler.rs          # Request handlers
│   │   ├── server.rs           # Server implementation
│   │   └── main.rs             # Entry point
│   └── resources/              # Game assets
│       └── www/                # Web resources (sprites, maps)
└── pixel_art/                  # Source art files
```

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/tower-defense-backend.git
   cd tower-defense-backend
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Starting the Server

```bash
cargo run --release --bin tower-defense-server
```

The server will start on `localhost:6767`.

### API Endpoints

- `GET /health` - Health check endpoint
- `GET /resources/*` - Static resources
- `GET /structures` - Available tower structures data
- `GET /enemies` - Enemy types data
- `GET /games` - List of saved games
- `WS /game/create` - Create a new game lobby
- `WS /game/join/{lobby_id}` - Join an existing game lobby

### WebSocket Messages

#### Client to Server:
- Start game
- Load saved game
- Place tower
- Upgrade tower
- Chat message
- Save game

#### Server to Client:
- Game updates
- Player list updates
- Chat messages
- Game state updates

## Game Mechanics

### Tower Defense Basics

Players defend against waves of enemies by strategically placing towers along the enemy path. Each tower has different abilities, costs, and upgrade paths.

### Towers

- **Lightning Tower**: Deals damage to enemies in a radius
- **Konfetti Kanone**: Area of effect damage with splash
- **Single Shot Tower**: High damage to a single target

Towers can be upgraded to more powerful versions with increased range, damage, or special abilities.

### Enemies

Enemies follow a predefined path and have different health, speed, and damage values. When defeated, they reward the player with coins.

### Economy

Players earn coins by defeating enemies. These coins can be used to build new towers or upgrade existing ones.

### Multiplayer

The game supports multiple players in a lobby with a host-client model. Only the host can start or load games, but all players can place towers and participate in the defense.
