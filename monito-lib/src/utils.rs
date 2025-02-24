use crate::Temperature;

pub fn convert_kelvin_to_rgb(kelvin: u16) -> Temperature {
    let t = kelvin as f64 / 100.0;

    let red_ratio = if t <= 66.0 {
        1.0
    } else {
        let red = 351.9769056680568 * (t - 60.0).powf(-0.1332047592);
        f64::max(0.0, f64::min(red / 255.0, 1.0))
    };

    let green_ratio = if t <= 66.0 {
        let green = 99.4708025861 * f64::ln(t) - 161.1195681661;
        f64::max(0.0, f64::min(green / 222.442822, 1.0))
    } else {
        let green = 138.5177312231 * f64::ln(t - 10.0) - 305.0447927307;
        f64::max(0.0, f64::min(green / 222.442822, 1.0))
    };

    let blue_ratio = if t >= 66.0 {
        1.0
    } else if t <= 19.0 {
        0.0
    } else {
        let blue = 138.5177312231 * f64::ln(t - 10.0) - 305.0447927307;
        f64::max(0.0, f64::min(blue / 400.0, 1.0))
    };

    // 计算增益值（0 - 100）
    let mut gain_r = (red_ratio * 100.0).round();
    let mut gain_g = (green_ratio * 100.0).round();
    let mut gain_b = (blue_ratio * 100.0).round();

    gain_r = f64::max(0.0, f64::min(gain_r, 100.0));
    gain_g = f64::max(0.0, f64::min(gain_g, 100.0));
    gain_b = f64::max(0.0, f64::min(gain_b, 100.0));

    println!("R: {}, G: {}, B: {}", gain_r, gain_g, gain_b);

    Temperature {
        temp: kelvin,
        rgb: (gain_r as u8, gain_g as u8, gain_b as u8),
    }
}
