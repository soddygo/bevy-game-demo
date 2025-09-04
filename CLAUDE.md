# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a 2D pixel-style farm simulation game built with Bevy 0.16. The game features randomly generated worlds with animals, vegetation, buildings, and day/night cycles. The core gameplay involves interacting with various farm animals, managing inventory, and exploring a procedurally generated world.

## Development Commands

### Building and Running
- `cargo run` - Build and run the game
- `cargo check` - Check for compilation errors
- `cargo build` - Build the project
- `cargo build --release` - Build optimized release version

### Testing
- `cargo test` - Run all tests (if any)
- `cargo test <test_name>` - Run specific test

## Architecture Overview

### Plugin-Based System Architecture
The game is organized around Bevy plugins, each responsible for a specific domain:

1. **PlayerPlugin** (`main.rs:12-22`) - Handles player spawning, movement, and animation
2. **AnimalsPlugin** (`main.rs:24-34`) - Manages animal spawning, movement, and AI behavior
3. **InteractionPlugin** (`main.rs:36-46`) - Handles player-animal interactions and prompts
4. **TimeOfDayPlugin** (`main.rs:48-60`) - Manages game time, day/night cycles, and visual effects
5. **InventoryPlugin** (`main.rs:62-74`) - Handles inventory management and item usage
6. **WorldPlugin** (`main.rs:76-82`) - Manages world generation (terrain, vegetation, buildings)
7. **CameraPlugin** (`camera.rs:3-10`) - Handles camera setup and player following

### Core Components (`src/components/mod.rs`)

#### Player Components
- `Player` - Movement speeds and player state
- `PlayerDirection` - Current facing direction
- `PlayerAnimation` - Animation state and timing

#### Animal Components
- `Animal` - Animal type, speed, friendship level, hunger
- `AnimalAI` - State machine for animal behavior (Idle, Wandering, Following, Eating)
- `Interactable` - Defines interaction types and ranges

#### World Systems
- `TimeOfDay` - Game time with hours, minutes, and phases
- `Inventory` - Item management with capacity and selection
- Various enums for animal types, items, and interactions

### Trait System (`src/traits/mod.rs`)

The game uses a trait-based approach for behavior:

1. **AnimalBehavior** - Defines animal-specific behaviors:
   - Favorite foods, personality traits, movement speeds
   - Friendship level requirements for following player
   - Different behaviors for each animal type (Dog, Cat, Chicken, Cow, Sheep, Pig, Duck, Horse)

2. **ItemUsable** - Defines how items interact with animals:
   - Which items can be used on which animals
   - Friendship bonuses and hunger reduction values
   - Item-animal compatibility matrix

3. **InteractableEntity** - Defines interaction mechanics:
   - Interaction prompts and validation
   - Different interaction types (Pet, Feed, Play)

### Random World Generation

The game features complete randomization of world elements:

#### Animal Generation (`src/systems/animal_systems.rs:8-80`)
- Random counts for each animal type (e.g., 3-8 dogs, 3-7 cows)
- Random positioning across the entire 8000x8000 map
- Each animal type has unique spawn functions with detailed pixel art

#### Vegetation Generation (`src/systems/world_systems.rs`)
- **Dense Grass**: 5 different grass types covering the entire map
- **Flowers**: 3000 randomly distributed flowers + 15 flower clusters
- **Shrubs**: 300 random shrubs + 20 shrub clusters
- **Mushrooms**: 150 random mushrooms + 25 mushroom clusters
- **Trees**: Random forest clusters (12-20 clusters) + scattered trees (50-80) + edge trees (30-50)

#### Building Generation
- Random number of houses (4-8) with random positions
- 30% chance for barns, 70% for houses
- Random house colors and building types

### Key Systems

#### Animal AI System
- State machine with Idle, Wandering, Following, and Eating states
- Movement boundaries expanded to match the 8000x8000 map
- Reduced idle time (1 second) for more active behavior
- Time-based behavior changes (morning activity, noon rest, evening activity, night sleep)

#### Time System
- Game time progresses faster than real time (1 second = 60 game minutes)
- Four day phases: Morning, Noon, Evening, Night
- Dynamic visual effects: background colors and lighting change with time
- Animal behavior changes based on time of day

#### Inventory System
- Tab-based inventory with 12-item capacity
- Arrow keys and number keys for item selection
- Item stacking and usage mechanics
- Integration with animal feeding system

#### Camera System
- Smooth camera following with interpolation
- 2D orthographic projection with fixed vertical viewport
- Component-based camera tagging

## Code Conventions

### Entity-Component-System (ECS) Pattern
- Use Bevy's ECS architecture throughout
- Components store data, systems handle logic
- Queries for accessing entity data efficiently
- Resources for global state (inventory, time of day)

### Random Number Generation
- Use `rand::rng()` for creating random number generators
- Random ranges use `rng.random_range(start..end)`
- All world elements are procedurally generated for variety

### Chinese Language Support
- Game uses Chinese text for animal names, personalities, and interactions
- UI prompts and feedback are in Chinese
- Maintain consistency with existing Chinese text

### Performance Considerations
- Map size is 8000x8000 units with dense vegetation
- Animals move within boundaries of -3400 to 3400 on both axes
- Z-layering used for proper rendering order (terrain at -10, UI at higher values)

## Important Files

- `src/main.rs` - Plugin registration and app setup
- `src/components/mod.rs` - All component definitions
- `src/traits/mod.rs` - Behavior trait implementations
- `src/systems/animal_systems.rs` - Animal spawning and AI
- `src/systems/world_systems.rs` - World generation and vegetation
- `src/systems/time_systems.rs` - Day/night cycle management
- `src/systems/inventory_systems.rs` - Inventory and item management
- `src/camera.rs` - Camera following and projection

## Dependencies

- **bevy = "0.16"** - Game engine and ECS framework
- **rand = "0.9"** - Random number generation for procedural content


## bevy 源码参考
* bevy源码,路径在: /Volumes/soddy/git_workspace/bevy ,可以参考