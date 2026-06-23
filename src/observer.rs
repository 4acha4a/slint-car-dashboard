use crate::Vehicle;

use crate::{AppWindow, CoolerState, DashboardState, FuelState, ShifterState, SpeedometerState, TachometerState};
#[cfg(feature = "web")]
use reqwest::Client;
use slint::{ComponentHandle, Timer, TimerMode};

use std::{sync::{Mutex, OnceLock}, time::{Duration, Instant}};

static VEHICLE_OBSERVER: OnceLock<Mutex<VehicleObserver>> = OnceLock::new();
static KEY_HOLD: OnceLock<Mutex<Option<String>>> = OnceLock::new();

struct VehicleObserver {
    ui: slint::Weak<AppWindow>,
    vehicle: Vehicle,
    #[cfg(feature = "web")]
    last_web_update: Instant,
}

impl VehicleObserver {
    fn new(ui: slint::Weak<AppWindow>, vehicle: Vehicle) -> VehicleObserver {
        VehicleObserver { ui, vehicle, #[cfg(feature = "web")] last_web_update: Instant::now() }
    }

    fn update(&mut self) {
        self.key_hold();
        self.vehicle.goggle_rate();
        if let Some(ui) = self.ui.upgrade() {
            println!("Updating");
            ui.global::<DashboardState>().set_is_engine_on(self.vehicle.is_on());
            ui.global::<FuelState>().set_fuel(self.vehicle.get_fuel());
            ui.global::<CoolerState>().set_temperature(self.vehicle.get_temperature() as i32);
            ui.global::<TachometerState>().set_rate(self.vehicle.get_rate());
            ui.global::<SpeedometerState>().set_speed(self.vehicle.get_speed() as i32);
            ui.global::<ShifterState>().set_current_gear(self.vehicle.get_gear_str().into());
            #[cfg(feature = "web")]
            if self.last_web_update.elapsed() >= Duration::from_millis(100) {
                self.last_web_update = Instant::now();

                let data = self.vehicle.get_data();

                tokio::spawn(async move {
                    if let Err(err) = VehicleObserver::send_vehicle_data(data).await {
                        eprintln!("Failed to send vehicle data: {err:?}");
                    }
                });
            }
        }
    }

    #[cfg(feature = "web")]
    async fn send_vehicle_data(data: crate::vehicle::VehicleData) -> Result<(), reqwest::Error> {
        let client = Client::new();

        let _response = client
            .post("http://127.0.0.1:3000/data")
            .json(&data)
            .send()
            .await?;

        //println!("POST status: {}", response.status());

        Ok(())
    }

    pub fn key_pressed(&mut self, key: &str) {
        match key {
            "w" => {
                let mut key_held = KEY_HOLD.get().unwrap().lock().unwrap();
                *key_held = Some("w".into());
            }
            "e" => {
                if self.vehicle.is_moving() {
                    print!("Can't toggle power while moving!");
                    return;
                }
                self.vehicle.toggle_power();
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
                self.vehicle.refuel(0.02);
            }
            _ => {
                self.vehicle.idle_brake();
            }
        }
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

fn start_timer() -> Timer {
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
    timer
}

pub fn run() {
    let window =AppWindow::new().unwrap();
    VEHICLE_OBSERVER.set(Mutex::new(VehicleObserver::new(window.as_weak(), Vehicle::default())))
        .map_err(|_| "VEHICLE_OBSERVER already initialized")
        .unwrap();
    KEY_HOLD.set(Mutex::new(None)).map_err(|_| "KEY_HOLD already initialized").unwrap();
    on_key_pressed(&window);
    on_key_released(&window);
    on_quit_requested(&window);
    let _timer = start_timer();
    window.run().unwrap();
}
