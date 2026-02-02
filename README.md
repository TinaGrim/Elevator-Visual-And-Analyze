# Elevator Visualization and Analysis

A visual elevator simulator built with Rust using the [egui](https://github.com/emilk/egui) immediate mode GUI framework.

## Description

This application provides a visual representation of an elevator system with:
- A graphical elevator display
- Multiple floors (Ground, 1, 2, 3)
- Real-time visualization
- Interactive controls to start/stop the simulation

## Features

- **Visual Elevator**: Displays an elevator with a custom image
- **Floor Display**: Shows floor labels (Ground, 1, 2, 3)
- **Grid Layout**: Visual grid representing the building structure
- **Interactive UI**: Start/stop controls for the elevator simulation

## Prerequisites

To build and run this project, you need:
- Rust (1.93.0 or later)
- Cargo (comes with Rust)
- X11 libraries (for GUI support)

### Installing Rust

If you don't have Rust installed, you can install it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing System Dependencies

On Ubuntu/Debian-based systems:

```bash
sudo apt-get update
sudo apt-get install -y libxkbcommon-x11-0 libxcb-xkb1
```

On other Linux distributions, you may need to install equivalent packages for X11 and XKB support.

## Building the Project

To build the project, run:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

## Running the Application

To run the elevator simulator:

```bash
cargo run
```

Or, if you've built the release version:

```bash
cargo run --release
```

The application will open a GUI window displaying the elevator visualization.

## Project Structure

```
.
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── src/
│   ├── main.rs            # Application entry point and main UI logic
│   ├── elevator.rs        # Elevator object and state management
│   ├── elevator_widget.rs # Custom widget for rendering the elevator
│   └── elevator.png       # Elevator image asset
└── README.md              # This file
```

## Dependencies

- `eframe` (0.27.0) - Framework for creating GUI applications
- `egui` (0.27.0) - Immediate mode GUI library
- `egui_extras` (0.27.0) - Additional utilities for egui, including image loading
- `rand` (0.8.5) - Random number generation

## Notes

- The application uses a custom elevator image (`src/elevator.png`)
- The simulation displays a grid with floor markers
- The elevator position and movement logic can be extended
- Currently in minimal working state with room for additional features

## License

This project is open source. Please refer to the repository for license information.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.
