# Elevator GUI - Playable Version

## What's New

The Elevator GUI is now fully interactive and playable! The elevator automatically moves between floors, simulating a real elevator system.

## Features Added

### 1. Automated Elevator Movement
- The elevator now moves smoothly between floors
- Automatically selects random target floors
- Visual animation shows the elevator traveling up and down

### 2. Interactive Controls
- **Start/Stop Button**: Click the "🕹️ Start" checkbox to begin/pause simulation
- **Floor Display**: Shows current floor and target floor (e.g., "Floor: G → 2")
- **Door Toggle**: Click directly on the elevator to open/close doors (see console output)

### 3. Technical Implementation
- **Position Tracking**: Separate tracking of elevator's actual position vs fixed floor positions
- **Smooth Animation**: 2.0 pixels/frame movement speed
- **Random Selection**: Uses the `rand` crate for random floor selection
- **State Management**: Properly tracks current floor, target floor, and elevator position

## How to Use

### Quick Start
```bash
# Using the provided script
./run_gui.sh

# Or directly with cargo
cargo run
```

### Controls
1. Launch the application
2. Click the "Start" checkbox to begin simulation
3. Watch the elevator move between floors automatically
4. Click on the elevator itself to toggle the door
5. Uncheck "Start" to pause the simulation

## System Requirements

- Rust 1.93.0 or later
- X11 libraries for GUI support
- OpenGL support

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y libxkbcommon-x11-0 libxcb-xkb1
```

## Project Structure

```
src/
├── main.rs           - Main application with simulation logic
├── elevator.rs       - Elevator object and state management
├── elevator_widget.rs - Custom widget for rendering
└── elevator.png      - Elevator image asset
```

## Building

```bash
# Development build
cargo build

# Optimized release build
cargo build --release
```

## Implementation Details

The elevator simulation works by:
1. Tracking the elevator's Y position on screen
2. Comparing current position with target floor position
3. Moving incrementally towards the target
4. Selecting a new random target when destination is reached

Floor positions are fixed at:
- Ground: 0.0
- Floor 1: 200.0
- Floor 2: 400.0
- Floor 3: 600.0

## Future Enhancements

Potential improvements:
- Manual floor selection buttons
- Multiple elevators
- Waiting passengers
- Door animation
- Sound effects
- Floor request queue
- Statistics tracking

## License

Open source - see repository for details.
