# Slint Vehicle Dashboard Simulator

A Rust + Slint desktop application that simulates a vehicle dashboard with speedometer, tachometer, fuel gauge, temperature gauge, and gear selector.

The project is built as a small interactive dashboard simulator. It models basic vehicle behavior such as engine startup, acceleration, braking, idle mode, fuel consumption, temperature changes, and gear-dependent RPM behavior.

## Features

- Interactive vehicle dashboard UI built with Slint
- Speedometer simulation
- Tachometer / RPM simulation
- Fuel gauge
- Engine temperature gauge
- Gear selector display
- Engine on/off logic
- Acceleration and braking model
- Idle behavior when no keys are pressed
- Fuel consumption
- Basic automatic gear behavior
- Background update loop using Slint timers

## Controls

| Key | Action |
|---|---|
| `E` | Turn engine on/off |
| `W` | Accelerate |
| `S` | Brake |
| `R` | Refuel |
| `Meta + C` | Quit application |

## Tech Stack

- Rust
- Slint
- rand crate

## Project Structure

```text
.
├── src
│   ├── main.rs
│   ├── vehicle
│   │   ├── mod.rs
│   │   └── model.rs
│   └── ui
│       ├── appwindow.slint
│       └── components
│           ├── speedometer.slint
│           ├── tachometer.slint
│           ├── fuel.slint
│           ├── cooler.slint
│           ├── shifter.slint
│           └── dashboard_state.slint
├── res
│   └── interior.png
├── Cargo.toml
└── README.md