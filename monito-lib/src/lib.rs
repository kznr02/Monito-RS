mod utils;
mod win;

use serde::{Deserialize, Serialize};
use win::{WindowsMonitor, enumerate_monitor_windows};
use windows::Win32::Devices::Display::MC_COLOR_TEMPERATURE;

trait InnerMonitorTrait {
    fn get_monitor_brightness(&self) -> Brightness;

    fn set_monitor_brightness(&self, val: u32);

    fn get_monitor_temperature(&self) -> u16;

    fn set_monitor_temperature(&self, kelvin: u16);

    // fn get_monitor_supported_vcp(&self);

    fn get_monitor_rgb(&self) -> RGBParam;

    fn set_monitor_rgb(&self, color: Color, val: u8);

    fn get_monitor_name(&self) -> String;
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Brightness {
    current: u32,
    min: u32,
    max: u32,
}

impl Brightness {
    pub fn new() -> Self {
        Brightness {
            current: 0,
            min: 0,
            max: 0,
        }
    }
}

#[derive(Serialize)]
struct Temperature {
    temp: u16,
    rgb: (u8, u8, u8),
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct ColorParam {
    min: u32,
    max: u32,
    current: u32,
}

impl ColorParam {
    fn new() -> ColorParam {
        ColorParam {
            min: 0,
            max: 0,
            current: 0,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct RGBParam {
    r: ColorParam,
    g: ColorParam,
    b: ColorParam,
}

impl RGBParam {
    fn new() -> RGBParam {
        RGBParam {
            r: ColorParam::new(),
            g: ColorParam::new(),
            b: ColorParam::new(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub struct Monitor {
    pub name: String,
    pub brightness: Brightness,
    pub temperature: u16,
    pub rgb: RGBParam,
    // supported_vcp: Box<Vec<u8>>,
    monitor_ins: Box<dyn InnerMonitorTrait>,
}

impl Monitor {
    pub fn get_monitor_brightness(&mut self) -> Brightness {
        self.brightness = self.monitor_ins.get_monitor_brightness();

        self.brightness
    }

    pub fn set_monitor_brightness(&self, val: u32) {
        self.monitor_ins.set_monitor_brightness(val);
    }

    pub fn get_monitor_temperature(&mut self) -> u16 {
        self.temperature = self.monitor_ins.get_monitor_temperature();

        self.temperature
    }

    pub fn set_monitor_temperature(&self, kelvin: u16) {
        self.monitor_ins.set_monitor_temperature(kelvin);
    }

    pub fn get_monitor_rgb(&mut self) -> RGBParam {
        self.rgb = self.monitor_ins.get_monitor_rgb();

        self.rgb
    }

    pub fn set_monitor_rgb(&self, color: Color, val: u8) {
        self.monitor_ins.set_monitor_rgb(color, val);
    }
}

pub fn enumerate_monitor() -> Vec<Monitor> {
    #[cfg(target_os = "windows")]
    {
        enumerate_monitor_windows()
            .iter()
            .map(|m| {
                let clean_name = m.get_monitor_name().trim_end_matches('\0').to_string();

                Monitor {
                    name: clean_name,
                    brightness: m.get_monitor_brightness(),
                    temperature: m.get_monitor_temperature(),
                    rgb: m.get_monitor_rgb(),
                    // supported_vcp: m.get_monitor_supported_vcp(),
                    monitor_ins: Box::new(m.clone()),
                }
            })
            .collect()
    }
    #[cfg(target_os = "linux")]
    {
        // Linux code here
    }
}
#[cfg(test)]
mod tests {
    use log::LevelFilter;

    #[test]
    fn test_enumerate_monitor() {
        let mut monitors = super::enumerate_monitor();

        // monitors.iter_mut().for_each(|m| {
        //     println!("Monitor: {:?}", m.name);
        //     println!("Brightness: {:?}", m.get_monitor_brightness());
        //     m.get_monitor_temperature();

        //     // m.set_monitor_temperature(4500);
        // });
    }
}
