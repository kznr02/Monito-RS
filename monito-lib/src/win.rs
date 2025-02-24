use log::*;
use windows::Win32::{
    Devices::Display::{
        GetMonitorBrightness, GetMonitorCapabilities, GetMonitorColorTemperature,
        GetMonitorRedGreenOrBlueDrive, GetMonitorRedGreenOrBlueGain,
        GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, MC_BLUE_DRIVE,
        MC_BLUE_GAIN, MC_COLOR_TEMPERATURE, MC_COLOR_TEMPERATURE_4000K,
        MC_COLOR_TEMPERATURE_10000K, MC_GREEN_GAIN, MC_RED_GAIN, PHYSICAL_MONITOR,
        SetMonitorBrightness, SetMonitorColorTemperature, SetMonitorRedGreenOrBlueGain,
    },
    Foundation::{BOOL, LPARAM, RECT, TRUE},
    Graphics::Gdi::{
        EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO, MONITORINFOEXW,
    },
};

use crate::{Brightness, Color, InnerMonitorTrait, RGBParam, utils::convert_kelvin_to_rgb};
#[derive(Clone, Copy)]
pub struct WindowsMonitor {
    hmonitor: HMONITOR,
    monitor_info: MONITORINFOEXW,
    physical_monitor: PHYSICAL_MONITOR,
}

unsafe extern "system" fn lpfnenum(
    hmonitor: HMONITOR,
    _hdc: HDC,
    _lprcmonitor: *mut RECT,
    dwdata: LPARAM,
) -> BOOL {
    unsafe {
        let monitors = &mut *(dwdata.0 as *mut Vec<WindowsMonitor>);
        let mut monitor_info = MONITORINFOEXW::default();
        let mut physical_monitors;
        let mut monitor_num = 0;
        let mut ret = TRUE;

        monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

        ret = GetMonitorInfoW(
            hmonitor,
            &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
        );

        debug!("GetMonitorInfoW: {:?}", ret);

        match GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut monitor_num) {
            Ok(_) => {}
            Err(e) => {
                error!("GetNumberOfPhysicalMonitorsFromHMONITOR: {:?}", e);
            }
        }

        physical_monitors = vec![PHYSICAL_MONITOR::default(); monitor_num as usize];

        match GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut physical_monitors) {
            Ok(_) => {}
            Err(e) => {
                error!("GetPhysicalMonitorsFromHMONITOR: {:?}", e);
            }
        }

        debug!("Start enum monitor");

        physical_monitors.iter().for_each(|m| {
            let monitor = WindowsMonitor {
                hmonitor,
                monitor_info,
                physical_monitor: m.clone(),
            };
            monitors.push(monitor);
        });

        TRUE
    }
}

pub(crate) fn enumerate_monitor_windows() -> Vec<WindowsMonitor> {
    let mut monitors = Vec::new();
    unsafe {
        let _ = EnumDisplayMonitors(
            None,
            None,
            Some(lpfnenum),
            LPARAM(&mut monitors as *mut Vec<WindowsMonitor> as isize),
        );
    }

    debug!("Len of monitors {:?}", monitors.len());

    monitors
}

impl InnerMonitorTrait for WindowsMonitor {
    fn get_monitor_brightness(&self) -> Brightness {
        let handle = self.physical_monitor.hPhysicalMonitor;
        let mut b = Brightness::new();
        unsafe {
            let ret = GetMonitorBrightness(handle, &mut b.min, &mut b.current, &mut b.max);

            if ret == 0 {
                debug!("Can't get brightness in MCCP, try WMI");

                todo!()
            }

            b
        }
    }

    fn set_monitor_brightness(&self, val: u32) {
        let handle = self.physical_monitor.hPhysicalMonitor;
        unsafe {
            SetMonitorBrightness(handle, val);
        }
    }

    fn get_monitor_temperature(&self) -> u16 {
        let handle = self.physical_monitor.hPhysicalMonitor;

        let mut temp = MC_COLOR_TEMPERATURE::default();

        unsafe {
            GetMonitorColorTemperature(handle, &mut temp as &mut MC_COLOR_TEMPERATURE);
        }

        0
    }

    fn set_monitor_temperature(&self, kelvin: u16) {
        let handle = self.physical_monitor.hPhysicalMonitor;
        let temp = convert_kelvin_to_rgb(kelvin);
        let mut ret = 0;
        unsafe {
            ret = SetMonitorRedGreenOrBlueGain(handle, MC_RED_GAIN, temp.rgb.0 as u32);
            if ret == 1 {
                error!("SetMonitorRedGreenOrBlueGain failed");
            }
            ret = SetMonitorRedGreenOrBlueGain(handle, MC_GREEN_GAIN, temp.rgb.1 as u32);
            if ret == 1 {
                error!("SetMonitorRedGreenOrBlueGain failed");
            }
            ret = SetMonitorRedGreenOrBlueGain(handle, MC_BLUE_GAIN, temp.rgb.2 as u32);
            if ret == 1 {
                error!("SetMonitorRedGreenOrBlueGain failed");
            }
        }
    }

    fn get_monitor_rgb(&self) -> RGBParam {
        let handle = self.physical_monitor.hPhysicalMonitor;
        let mut rgb = RGBParam::new();
        unsafe {
            GetMonitorRedGreenOrBlueGain(
                handle,
                MC_RED_GAIN,
                &mut rgb.r.min,
                &mut rgb.r.current,
                &mut rgb.r.max,
            );

            GetMonitorRedGreenOrBlueGain(
                handle,
                MC_GREEN_GAIN,
                &mut rgb.g.min,
                &mut rgb.g.current,
                &mut rgb.g.max,
            );

            GetMonitorRedGreenOrBlueGain(
                handle,
                MC_BLUE_GAIN,
                &mut rgb.b.min,
                &mut rgb.b.current,
                &mut rgb.b.max,
            );
        }

        rgb
    }

    fn set_monitor_rgb(&self, color: Color, val: u8) {
        let handle = self.physical_monitor.hPhysicalMonitor;
        unsafe {
            match color {
                Color::Red => {
                    SetMonitorRedGreenOrBlueGain(handle, MC_RED_GAIN, val as u32);
                }
                Color::Green => {
                    SetMonitorRedGreenOrBlueGain(handle, MC_GREEN_GAIN, val as u32);
                }
                Color::Blue => {
                    SetMonitorRedGreenOrBlueGain(handle, MC_BLUE_GAIN, val as u32);
                }
            }
        }
    }

    fn get_monitor_name(&self) -> String {
        String::from_utf16_lossy(&self.monitor_info.szDevice)
    }
}

impl WindowsMonitor {
    fn parse_capbility() {}
}

// fn get_monitor_supported_vcp_windows(monitor: &mut Monitor) {
//     #[cfg(target_os = "windows")]
//     {
//         let mut ddc_len = 0;
//         let mut vcp_reply = vec![0u8; 128];
//         unsafe {
//             GetCapabilitiesStringLength(monitor.physical_monitor.hPhysicalMonitor, &mut ddc_len);
//             println!("CapabilitiesStringLength: {:?}", ddc_len);
//             let len = ddc_len as usize;
//             vcp_reply = vec![0u8; len];
//             let ret = CapabilitiesRequestAndCapabilitiesReply(
//                 monitor.physical_monitor.hPhysicalMonitor,
//                 &mut vcp_reply,
//             );
//             println!("CapabilitiesRequestAndCapabilitiesReply: {:?}", ret);
//             monitor.supported_vcp = vcp_reply.to_vec();
//         }
//         println!("VCP: {:?}", String::from_utf8_lossy(&vcp_reply));
//     }
// }
