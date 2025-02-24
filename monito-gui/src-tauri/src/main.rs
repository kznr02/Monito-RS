// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, Once};

use log::error;
use monito_lib::{enumerate_monitor, Brightness, Color, Monitor, RGBParam};
use serde::{Deserialize, Serialize};

static mut MONITOR_INSTANCES: Option<Mutex<Vec<Monitor>>> = None;
static INIT: Once = Once::new();

#[derive(Serialize)]
struct MonitorPara {
    pub name: String,
    pub path: String,
}

fn init_monitor_instances() {
    INIT.call_once(|| {
        let instances = enumerate_monitor();
        unsafe {
            MONITOR_INSTANCES = Some(Mutex::new(instances));
        }
    });
}

#[tauri::command]
fn get_monitor_inst_simple_list() -> Option<Vec<MonitorPara>> {
    let list;

    unsafe {
        match &MONITOR_INSTANCES {
            Some(ms) => {
                if let Ok(instances) = ms.lock() {
                    list = instances
                        .iter()
                        .map(|m| MonitorPara {
                            name: m.name.clone(),
                            path: m.name.clone(),
                        })
                        .collect();

                    return Some(list);
                } else {
                    error!("Instance busy, get failed");

                    return None;
                }
            }
            None => {
                error!("MONITOR_INSTANCES is null");
                return None;
            }
        }
    }
}

#[tauri::command]
fn get_monitor_brightness(index: u8) -> Option<Brightness> {
    unsafe {
        match &MONITOR_INSTANCES {
            Some(ms) => {
                if let Ok(mut instances) = ms.lock() {
                    return Some(instances[index as usize].get_monitor_brightness());
                } else {
                    return None;
                }
            }
            None => None,
        }
    }
}

#[derive(Deserialize)]
struct BrightnessSetter {
    index: u8,
    value: u8,
}

#[tauri::command]
fn set_monitor_brightness(params: BrightnessSetter) {
    println!("set {:?}", params.value);
    unsafe {
        match &MONITOR_INSTANCES {
            Some(ms) => {
                if let Ok(instances) = ms.lock() {
                    instances[params.index as usize].set_monitor_brightness(params.value as u32);
                } else {
                    ()
                }
            }
            None => (),
        }
    }
}

#[tauri::command]
fn get_monitor_color(index: usize) -> Option<RGBParam> {
    unsafe {
        match &MONITOR_INSTANCES {
            Some(ms) => {
                if let Ok(mut instances) = ms.lock() {
                    return Some(instances[index].get_monitor_rgb());
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ColorSetter {
    index: usize,
    color: Color,
    value: u8,
}

#[tauri::command]
fn set_monitor_color(params: ColorSetter) {
    unsafe {
        match &MONITOR_INSTANCES {
            Some(ms) => {
                if let Ok(m) = ms.lock() {
                    m[params.index].set_monitor_rgb(params.color, params.value);
                } else {
                    ()
                }
            }
            None => (),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_monitor_inst_simple_list,
            get_monitor_brightness,
            set_monitor_brightness,
            get_monitor_color,
            set_monitor_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    env_logger::builder()
        .parse_env("MONITO_DEBUG")
        .filter_level(log::LevelFilter::Info)
        .format_line_number(true)
        .build();

    init_monitor_instances();

    run();
}
