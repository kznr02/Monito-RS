import { invoke } from "@tauri-apps/api/core";

export const getMonitorBrightness = async (index) => {
  return invoke("get_monitor_brightness", { index: index });
};

export const updateBrightness = async (select, newValue) => {
  await invoke("set_monitor_brightness", {
    params: {
      index: select,
      value: newValue,
    },
  });
};

export const getMonitorColor = async (index) => {
  return invoke("get_monitor_color", { index: index });
};

export const setMonitorColor = async (index, color, value) => {
  await invoke("set_monitor_color", {
    params: { index: index, color: color, value: value },
  });
};
