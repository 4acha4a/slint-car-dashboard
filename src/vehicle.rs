pub const IDLE_RATE: f32 = 0.8;
pub const MAX_RATE: f32 = 5.5;
const DT: f32 = 0.016;

const MAX_FUEL: f32 = 100.0;
const INITIAL_TEMPERATURE: f32 = 70.0;

#[derive(Clone, Debug, PartialEq)]
pub enum ShifterGear {
    Parking,
    Neutral,
    Drive(u8),
}

pub struct Vehicle {
    fuel: f32,
    temperature: f32,
    gear: ShifterGear,
    speed: f32,
    rate: f32,
    is_on: bool,
    throttle: f32,
    brake: f32,
}

impl Vehicle {
    pub fn new() -> Self {
        Vehicle { fuel: MAX_FUEL, temperature: INITIAL_TEMPERATURE, gear: ShifterGear::Parking, speed: 0.0, throttle: 0.0, rate: 0.0, brake: 0.0, is_on: false }
    }

    pub fn consume_fuel(&mut self, amount: f32) {
        if self.fuel >= amount {
            self.fuel -= amount;
        } else {
            self.fuel = 0.0;
        }
    }

    pub fn refuel(&mut self, amount: f32) {
        if self.get_fuel() >= MAX_FUEL {
            println!("Cannot refuel: fuel tank is full");
            return;
        }
        self.fuel += amount;
        if self.fuel > MAX_FUEL {
            self.fuel = MAX_FUEL;
        }
    }

    pub fn get_fuel(&self) -> f32 {
        self.fuel
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn out_of_fuel(&self) -> bool {
        self.fuel == 0.0
    }

    pub fn ignition(&mut self) {
        if self.is_on() {
            self.turn_off();
            println!("Vehicle turned off");
        }
        else {
            if self.get_fuel() == 0.0 {
                println!("Cannot turn on: fuel tank is empty");
                return;
            }
            self.turn_on();
            println!("Vehicle turned on");
        }
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
        self.speed = 0.0;
        self.rate = 0.0;
        self.gear = ShifterGear::Parking;
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_gear_str(&self) -> &str {
        match self.gear {
            ShifterGear::Parking => "P",
            ShifterGear::Neutral => "N",
            ShifterGear::Drive(d) => match d {
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                _ => "?",
            },
        }
    }

    pub fn get_gear(&self) -> &ShifterGear {
        &self.gear
    }

    pub fn is_drive(&self) -> bool {
        self.gear != ShifterGear::Parking && self.gear != ShifterGear::Neutral
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }

    pub fn get_rate(&self) -> f32 {
        self.rate
    }

    pub fn shift_gear_up(&mut self) {
        self.gear = match self.gear {
            ShifterGear::Parking => ShifterGear::Drive(1),
            ShifterGear::Neutral => ShifterGear::Drive(1),
            ShifterGear::Drive(n) if n < 6 => ShifterGear::Drive(n + 1),
            _ => self.gear.clone(),
        };
    }

    pub fn shift_gear_down(&mut self) {
        self.gear = match self.gear {
            ShifterGear::Drive(n) if n > 1 => ShifterGear::Drive(n - 1),
            ShifterGear::Neutral => ShifterGear::Parking,
            _ => ShifterGear::Parking,
        };
    }

    pub fn accelerate(&mut self) {
        if !self.is_on() {
            return;
        }
        if self.out_of_fuel() {
            self.turn_off();
            return;
        }

        self.throttle += 3.0 * DT;

        if self.throttle > 1.0 {
            self.throttle = 1.0;
        }

        self.brake -= 4.0 * DT;

        if self.brake < 0.0 {
            self.brake = 0.0;
        }

        self.consume_fuel(0.0005 + self.throttle * 0.0015);
        let target_rate = match self.gear {
            ShifterGear::Neutral | ShifterGear::Parking => {
                IDLE_RATE + self.throttle * 4.0
            }

            ShifterGear::Drive(1) => {
                IDLE_RATE + self.throttle * 4.3
            }

            ShifterGear::Drive(2) => {
                IDLE_RATE + self.speed * 0.105
            }

            ShifterGear::Drive(3) => {
                IDLE_RATE + self.speed * 0.08
            }

            ShifterGear::Drive(4) => {
                IDLE_RATE + self.speed * 0.03
            }

            ShifterGear::Drive(5) => {
                IDLE_RATE + self.speed * 0.023
            }

            ShifterGear::Drive(6) => {
                IDLE_RATE + self.speed * 0.011
            }

            _ => {
                IDLE_RATE + self.speed * 0.02
            }
        };

        self.rate += (target_rate - self.rate) * 8.0 * DT;

        if self.rate < IDLE_RATE {
            self.rate = IDLE_RATE;
        }

        if self.rate > MAX_RATE {
            self.rate = MAX_RATE;
        }

        if self.is_drive() {
            let acceleration = match self.gear {
                ShifterGear::Drive(1) => 32.0,
                ShifterGear::Drive(2) => 26.0,
                ShifterGear::Drive(3) => 20.0,
                ShifterGear::Drive(4) => 15.0,
                ShifterGear::Drive(5) => 11.0,
                ShifterGear::Drive(6) => 8.0,
                _ => 0.0,
            };

            self.speed += self.throttle * acceleration * DT;
        }

        println!(
            "Accelerating: fuel={}, temp={}, gear={:?}, speed={}, rate={}, throttle={}, brake={}",
            self.fuel,
            self.temperature,
            self.gear,
            self.speed,
            self.rate,
            self.throttle,
            self.brake
        );
    }

    pub fn brake(&mut self) {
        if !self.is_on() {
            return;
        }

        self.brake += 0.15;

        if self.brake > 1.0 {
            self.brake = 1.0;
        }

        self.throttle -= 3.0 * DT;

        if self.throttle < 0.0 {
            self.throttle = 0.0;
        }

        let brake_power = 45.0;

        self.speed -= self.brake * brake_power * DT;

        if self.speed < 0.0 {
            self.speed = 0.0;
        }

        let target_rate = match self.gear {
            ShifterGear::Parking | ShifterGear::Neutral => IDLE_RATE,

            ShifterGear::Drive(1) => IDLE_RATE + self.speed * 0.08,
            ShifterGear::Drive(2) => IDLE_RATE + self.speed * 0.055,
            ShifterGear::Drive(3) => IDLE_RATE + self.speed * 0.04,
            ShifterGear::Drive(4) => IDLE_RATE + self.speed * 0.03,
            ShifterGear::Drive(5) => IDLE_RATE + self.speed * 0.02,
            ShifterGear::Drive(6) => IDLE_RATE + self.speed * 0.015,

            _ => IDLE_RATE,
        };

        self.rate += (target_rate - self.rate) * 5.0 * DT;

        if self.rate < IDLE_RATE && self.is_on() {
            self.rate = IDLE_RATE;
        }

        if self.speed == 0.0 && self.is_on() {
            self.rate = IDLE_RATE;
        }
    }

    pub fn idle_brake(&mut self) {
        if !self.is_on() {
            self.rate -= 3.0 * DT;

            if self.rate < 0.0 {
                self.rate = 0.0;
            }

            self.speed -= 4.0 * DT;

            if self.speed < 0.0 {
                self.speed = 0.0;
            }

            return;
        }

        self.throttle -= 1.5 * DT;

        if self.throttle < 0.0 {
            self.throttle = 0.0;
        }

        let deceleration = match self.gear {
            ShifterGear::Parking => 20.0,
            ShifterGear::Neutral => 2.0,

            ShifterGear::Drive(1) => 6.0,
            ShifterGear::Drive(2) => 5.0,
            ShifterGear::Drive(3) => 4.0,
            ShifterGear::Drive(4) => 3.0,
            ShifterGear::Drive(5) => 2.5,
            ShifterGear::Drive(6) => 2.0,

            _ => 3.0,
        };

        self.speed -= deceleration * DT;

        if self.speed < 0.0 {
            self.speed = 0.0;
        }

        let target_rate = match self.gear {
            ShifterGear::Parking | ShifterGear::Neutral => IDLE_RATE,

            ShifterGear::Drive(1) => IDLE_RATE + self.speed * 0.08,
            ShifterGear::Drive(2) => IDLE_RATE + self.speed * 0.055,
            ShifterGear::Drive(3) => IDLE_RATE + self.speed * 0.04,
            ShifterGear::Drive(4) => IDLE_RATE + self.speed * 0.03,
            ShifterGear::Drive(5) => IDLE_RATE + self.speed * 0.02,
            ShifterGear::Drive(6) => IDLE_RATE + self.speed * 0.015,

            _ => IDLE_RATE,
        };

        self.rate += (target_rate - self.rate) * 4.0 * DT;

        if self.rate <= IDLE_RATE {
            self.rate = IDLE_RATE;
        }

        if self.rate >= MAX_RATE {
            self.rate = MAX_RATE;
        }

        self.consume_fuel(0.00005);

        if self.out_of_fuel() {
            self.turn_off();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vehicle_init() {
        let vehicle = Vehicle::new();

        assert_eq!(vehicle.get_fuel(), MAX_FUEL);
        assert_eq!(vehicle.get_temperature(), INITIAL_TEMPERATURE);
        assert_eq!(vehicle.get_gear_str(), "P");
        assert_eq!(vehicle.get_speed(), 0.0);
        assert_eq!(vehicle.is_on(), false);
    }

    #[test]
    fn test_vehicle_ignition() {
        let mut vehicle = Vehicle::new();

        vehicle.ignition();
        assert_eq!(vehicle.is_on(), true);

        vehicle.ignition();
        assert_eq!(vehicle.is_on(), false);
    }

    #[test]
    fn test_vehicle_refuel() {
        let mut vehicle = Vehicle::new();

        vehicle.consume_fuel(50.0);
        assert_eq!(vehicle.get_fuel(), MAX_FUEL - 50.0);

        vehicle.refuel(30.0);
        assert_eq!(vehicle.get_fuel(), MAX_FUEL - 20.0);

        vehicle.refuel(30.0);
        assert_eq!(vehicle.get_fuel(), MAX_FUEL);
    }

    #[test]
    fn test_vehicle_accelerate() {
        let mut vehicle = Vehicle::new();
        vehicle.ignition();
        vehicle.shift_gear_up();
        assert_eq!(vehicle.get_speed(), 0.0);
        vehicle.accelerate();
        assert!(vehicle.get_speed() > 0.0);
        assert!(vehicle.get_fuel() < MAX_FUEL);
    }

    #[test]
    fn test_vehicle_brake() {
        let mut vehicle = Vehicle::new();
        vehicle.ignition();
        vehicle.shift_gear_up();
        vehicle.accelerate();
        assert!(vehicle.get_speed() > 0.0);
        let old_speed = vehicle.get_speed();
        vehicle.brake();
        assert!(vehicle.get_speed() < old_speed);
        assert!(vehicle.get_fuel() < MAX_FUEL);
    }

    #[test]
    fn test_vehicle_idle_brake() {
        let mut vehicle = Vehicle::new();
        vehicle.ignition();
        vehicle.shift_gear_up();
        vehicle.accelerate();
        assert!(vehicle.get_speed() > 0.0);
        let old_speed = vehicle.get_speed();
        vehicle.idle_brake();
        assert!(vehicle.get_speed() < old_speed);
        assert!(vehicle.get_fuel() < MAX_FUEL);
    }

    #[test]
    fn test_vehicle_shift_gear() {
        let mut vehicle = Vehicle::new();
        vehicle.ignition();
        assert_eq!(vehicle.get_gear_str(), "P");
        vehicle.shift_gear_up();
        assert_eq!(vehicle.get_gear_str(), "1");
        vehicle.shift_gear_up();
        assert_eq!(vehicle.get_gear_str(), "2");
        vehicle.shift_gear_down();
        assert_eq!(vehicle.get_gear_str(), "1");
        vehicle.shift_gear_down();
        assert_eq!(vehicle.get_gear_str(), "P");
    }
}