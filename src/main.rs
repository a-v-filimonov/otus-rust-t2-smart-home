#![allow(dead_code)]

struct SmartSocket {
    description: String,
    is_enable: bool,
    curr_power_consumption: Option<f64>,
}

impl SmartSocket {
    fn new(description: &str) -> Self {
        Self {
            description: String::from(description),
            is_enable: false,
            curr_power_consumption: None,
        }
    }

    fn toogle(&mut self) -> bool {
        match self.is_enable {
            true => {
                self.poweroff();
            }
            false => {
                self.poweron();
            }
        }
        self.is_enable
    }

    fn poweron(&mut self) {
        self.is_enable = true;
        self.curr_power_consumption = Some(0.0);
    }

    fn poweroff(&mut self) {
        self.is_enable = false;
        self.curr_power_consumption = None;
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

struct Thermometer {
    curr_temperature: Option<f64>,
}

impl Thermometer {
    fn new() -> Self {
        Self {
            curr_temperature: None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
