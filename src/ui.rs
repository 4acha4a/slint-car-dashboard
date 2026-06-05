use crate::{Vehicle, vehicle::{IDLE_RATE, MAX_RATE}};
use rand::{RngExt};

use crate::{AppWindow, CoolerState, DashboardState, FuelState, ShifterState, SpeedometerState, TachometerState};
use slint::{ComponentHandle, Timer, TimerMode};

use std::{sync::{Mutex, OnceLock}, time::Duration};

static VEHICLE_OBSERVER: OnceLock<Mutex<VehicleObserver>> = OnceLock::new();
static KEY_HOLD: OnceLock<Mutex<Option<String>>> = OnceLock::new();

struct VehicleObserver {
    ui: slint::Weak<AppWindow>,
    vehicle: Vehicle,
}

impl VehicleObserver {
    fn new(ui: slint::Weak<AppWindow>, vehicle: Vehicle) -> VehicleObserver {
        VehicleObserver { ui, vehicle }
    }

    fn limit_rate(&mut self, rate: f32, target: f32) {
        const TOLERANCE: f32 = 0.025;
        if (rate >= target - TOLERANCE) && (rate <= target + TOLERANCE) {
            let num = rand::rng().random_range(rate * (1.0 - TOLERANCE)..rate * (1.0 + TOLERANCE));
            self.vehicle.set_rate(num);
        }
    }

    fn update(&mut self) {
        self.key_hold();
        self.goggle_rate();
        if let Some(ui) = self.ui.upgrade() {
            ui.global::<DashboardState>().set_is_engine_on(self.vehicle.is_on());
            ui.global::<FuelState>().set_fuel(self.vehicle.get_fuel());
            ui.global::<CoolerState>().set_temperature(self.vehicle.get_temperature() as i32);
            ui.global::<TachometerState>().set_rate(self.vehicle.get_rate());
            ui.global::<SpeedometerState>().set_speed(self.vehicle.get_speed() as i32);
            ui.global::<ShifterState>().set_current_gear(self.vehicle.get_gear_str().into());
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        match key {
            "w" => {
                let mut key_held = KEY_HOLD.get().unwrap().lock().unwrap();
                *key_held = Some("w".into());
            }
            "e" => {
                self.vehicle.ignition();
            }
            "s" => {
                let mut key_held = KEY_HOLD.get().unwrap().lock().unwrap();
                *key_held = Some("s".into());
            }
            "r" => {
                let mut key_held = KEY_HOLD.get().unwrap().lock().unwrap();
                *key_held = Some("r".into());
            }
            "shift" => {
                if !self.vehicle.is_on() {
                    return;
                }
                self.vehicle.shift_gear_up();
            }
            "alt" => {
                if !self.vehicle.is_on() {
                    return;
                }
                self.vehicle.shift_gear_down();
            }
            _ => {}
        }
    }

    pub fn key_released(&mut self) {
        let mut key_held = KEY_HOLD.get().unwrap().lock().unwrap();
        *key_held = None;
    }

    fn key_hold(&mut self) {
        match *KEY_HOLD.get().unwrap().lock().unwrap() {
            Some(ref key) if key == "w" => {
                self.vehicle.accelerate();
            }
            Some(ref key) if key == "s" => {
                self.vehicle.brake();
            }
            Some(ref key) if key == "r" => {
                self.vehicle.refuel(0.1);
            }
            _ => {
                self.vehicle.idle_brake();
            }
        }
    }

    fn goggle_rate(&mut self) {
        if !self.vehicle.is_on() {
            return;
        }
        self.limit_rate(self.vehicle.get_rate(), IDLE_RATE);
        self.limit_rate(self.vehicle.get_rate(), MAX_RATE);
    }
}

fn on_key_pressed(ui: &AppWindow) {
    ui.on_key_pressed(move |key: slint::SharedString| {
        let mut observer = VEHICLE_OBSERVER
            .get()
            .expect("VehicleObserver not initialized")
            .lock()
            .unwrap();
        observer.key_pressed(key.as_str());
    });
}

fn on_key_released(ui: &AppWindow) {
    ui.on_key_released(move || {
        let mut observer = VEHICLE_OBSERVER
            .get()
            .expect("VehicleObserver not initialized")
            .lock()
            .unwrap();
        observer.key_released();
    });
}


fn on_quit_requested(ui: &AppWindow) {
    ui.on_quit_requested(move || {
        println!("^C");
        std::process::exit(0);
    });
}

fn start_timers() -> [Timer; 1] {
    let timer = Timer::default();
    timer.start(
        TimerMode::Repeated,
        Duration::from_millis(16),
        move || {
            let mut observer = VEHICLE_OBSERVER
                .get()
                .expect("VehicleObserver not initialized")
                .lock()
                .unwrap();
            observer.update();
        },
    );
    [timer]
}

pub fn run() {
    let window =AppWindow::new().unwrap();
    VEHICLE_OBSERVER.set(Mutex::new(VehicleObserver::new(window.as_weak(), Vehicle::new())))
        .map_err(|_| "VEHICLE_OBSERVER already initialized")
        .unwrap();
    KEY_HOLD.set(Mutex::new(None)).map_err(|_| "KEY_HOLD already initialized").unwrap();
    on_key_pressed(&window);
    on_key_released(&window);
    on_quit_requested(&window);
    let _timers = start_timers();
    window.run().unwrap();
}
