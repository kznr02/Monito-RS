#![allow(unused)]

use std::ptr::null_mut;

use windows::Win32::{
    Devices::Display::*,
    Foundation::{BOOL, LPARAM, RECT, TRUE},
    Graphics::Gdi::{
        DISPLAY_DEVICE_ATTACHED_TO_DESKTOP, DISPLAY_DEVICEW, EnumDisplayDevicesW,
        EnumDisplayMonitors, GetMonitorInfoA, GetMonitorInfoW, HDC, HMONITOR, LPD_SHARE_STENCIL,
        MONITOR_DEFAULTTOPRIMARY, MONITORINFO, MONITORINFOEXW, MonitorFromWindow,
    },
    UI::WindowsAndMessaging::GetDesktopWindow,
};

enum VcpCommands {
    Brightness = 0x10,
    Contrast = 0x12,

    ColorPreset = 0x14,
}
// 转换u8为VcpCommands
impl From<u8> for VcpCommands {
    fn from(value: u8) -> Self {
        match value {
            0x10 => VcpCommands::Brightness,
            0x12 => VcpCommands::Contrast,
            0x14 => VcpCommands::ColorPreset,
            _ => panic!("Invalid VCP command"),
        }
    }
}

pub struct Monitor {
    pub name: String,
    // #[cfg(target_os = "windows")]
    // handle: HMONITOR,
    supported_vcp: Vec<u8>,
    #[cfg(target_os = "windows")]
    physical_monitor: PHYSICAL_MONITOR,
}

impl Monitor {
    fn new() -> Self {
        Monitor {
            name: String::new(),
            supported_vcp: Vec::new(),
            physical_monitor: PHYSICAL_MONITOR::default(),
        }
    }
}

unsafe extern "system" fn lpfnenum(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _lprcmonitor: *mut RECT,
    dwdata: LPARAM,
) -> BOOL { unsafe {
    let monitors = &mut *(dwdata.0 as *mut Vec<Monitor>);
    let mut monitor_info = MONITORINFOEXW::default();
    let mut physical_monitors = vec![PHYSICAL_MONITOR::default(); 1];
    let mut monitor_num = 0;

    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;
    unsafe {
        GetMonitorInfoW(
            hmonitor,
            &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
        );
        GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut monitor_num);
        GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors);
    }

    for m in physical_monitors.iter_mut() {
        let monitor = Monitor {
            name: String::from_utf16_lossy(&monitor_info.szDevice),
            supported_vcp: Vec::new(),
            physical_monitor: m.clone(),
        };

        monitors.push(monitor);
    }

    TRUE
}}

pub fn enumerate_monitor() -> Box<Vec<Monitor>> {
    let mut monitors = Vec::new();
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let _ = EnumDisplayMonitors(
                None,
                None,
                Some(lpfnenum),
                LPARAM(&mut monitors as *mut Vec<Monitor> as isize),
            );
        }
    }
    #[cfg(target_os = "linux")]
    {
        // Linux code here
    }

    Box::new(monitors)
}

pub fn get_monitor_supported_vcp(monitor: &mut Monitor) {
    #[cfg(target_os = "windows")]
    {
        let mut ddc_len = 0;
        let mut vcp_reply = vec![0u8; 128];
        unsafe {
            GetCapabilitiesStringLength(monitor.physical_monitor.hPhysicalMonitor, &mut ddc_len);
            println!("CapabilitiesStringLength: {:?}", ddc_len);
            let len = ddc_len as usize;
            vcp_reply = vec![0u8; len];
            let ret = CapabilitiesRequestAndCapabilitiesReply(
                monitor.physical_monitor.hPhysicalMonitor,
                &mut vcp_reply,
            );
            println!("CapabilitiesRequestAndCapabilitiesReply: {:?}", ret);
            monitor.supported_vcp = vcp_reply.to_vec();
        }
        println!("VCP: {:?}", String::from_utf8_lossy(&vcp_reply));
    }
}

pub fn get_monitor_brightness(monitor: &mut Monitor) -> u32 {
    let mut brightness = 0;
    let mut cur_brightness = 0;
    let mut max_brightness = 0;
    let mut monitor_num = 0;

    unsafe {
        GetMonitorBrightness(
            monitor.physical_monitor.hPhysicalMonitor,
            &mut brightness,
            &mut cur_brightness,
            &mut max_brightness,
        );
    }

    cur_brightness
}

pub fn set_monitor_brightness(monitor: &mut Monitor, brightness: u32) {
    unsafe {
        SetMonitorBrightness(monitor.physical_monitor.hPhysicalMonitor, brightness);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_brightness() {
        let mut monitores = enumerate_monitor();
        for monitor in monitores.iter_mut() {
            get_monitor_brightness(monitor);

            set_monitor_brightness(monitor, 0);
        }
    }

    #[test]
    fn test_get_capabilities() {
        let mut monitores = enumerate_monitor();
        for monitor in monitores.iter_mut() {
            get_monitor_supported_vcp(monitor);
        }
    }
}
