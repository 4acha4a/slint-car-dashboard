# Slint Vehicle Dashboard Simulator

A Rust + Slint desktop application that simulates a vehicle dashboard with speedometer, tachometer, fuel gauge, temperature gauge, and gear selector.

The project is built as a small interactive dashboard simulator. It models basic vehicle behavior such as engine startup, acceleration, braking, idle mode, fuel consumption, temperature changes, and gear-dependent RPM behavior.

## Screenshot
![Vehicle dashboard screenshot](screenshot.png)

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
- Optional asynchronous telemetry streaming to a local Rust backend using the `web` Cargo feature

## Roadmap

- [x] Add asynchronous telemetry streaming to a Rust backend
- [ ] Add a web dashboard for live vehicle metrics
- [ ] Improve the RPM and gear-shifting simulation model
- [x] Add unit tests for the vehicle simulation logic
- [ ] Add embedded portability notes
- [ ] Add a demo GIF showing dashboard interaction

## Controls

| Key | Action |
|---|---|
| `E` | Turn engine on/off |
| `W` | Accelerate |
| `S` | Brake |
| `R` | Refuel |
| `Meta + C` | Quit application |

## Run
```sh
cargo run
```

## Web feature

The optional web feature enables asynchronous telemetry streaming to a local Rust backend.

Run the project with web support enabled:

```sh
cargo run --features web
```

By default, the backend is available at:

```
http://127.0.0.1:3000
```

Available endpoints:

| Method | Endpoint | Description |
|---|---|---|
| ```POST``` | ```/data``` | Receives the latest vehicle telemetry from the simulator|
| ```GET``` | ```/data``` | Returns the latest available vehicle telemetry

Example:
```sh
curl http://127.0.0.1:3000/data
```

The telemetry payload contains the current vehicle state: speed, RPM, fuel level, temperature, engine state, and selected gear.

## Tech Stack

- Rust
- Slint
- rand crate
- Tokio
- Axum
- Serde
- Reqwest

## Project Structure

```text
.
├── build.rs
├── Cargo.toml
├── LICENSE.txt
├── README.md
└── src
    ├── main.rs
    ├── res
    │   ├── cooler.png
    │   ├── fuel.png
    │   └── interior.png
    ├── slint
    │   ├── components
    │   │   ├── cooler.slint
    │   │   ├── dashboard_state.slint
    │   │   ├── fuel.slint
    │   │   ├── gauge_base
    │   │   │   ├── big_gauge.slint
    │   │   │   ├── data_dash.slint
    │   │   │   └── small_gauge.slint
    │   │   ├── shifter.slint
    │   │   ├── speedometer.slint
    │   │   └── tachometer.slint
    │   └── main.slint
    ├── observer.rs
    ├── web.rs
    └── vehicle.rs
```
