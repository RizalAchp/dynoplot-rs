extern crate plotters;

mod dynoplot;
use dynoplot::{rs_save_plot_dyno, rs_save_plot_dyno_all, PlotDynoSetting};
use std::os::raw::{c_char, c_float, c_int};
use std::{ffi::CStr, path::PathBuf};

#[no_mangle]
pub extern "C" fn DYNOPLOT_save_plot(
    raw_x: *const c_float,
    raw_y: *const c_float,
    raw_size: c_int,
) -> c_int {
    let result = std::panic::catch_unwind(|| {
        let size_n = raw_size as usize;
        let slice_x = unsafe { std::slice::from_raw_parts(raw_x, size_n) }.to_vec();
        let slice_y = unsafe { std::slice::from_raw_parts(raw_y, size_n) }.to_vec();
        let conf = PlotDynoSetting::current();
        rs_save_plot_dyno(&conf, slice_x, slice_y, size_n);
    });
    match result {
        Ok(_) => 0,
        Err(_) => {
            PlotDynoSetting::current().err_callback("PLOTERROR: Error on DYNOPLOT_save_plot");
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn DYNOPLOT_init_config(
    raw_path: *const c_char,
    raw_name: *const c_char,
    raw_xmin: c_float,
    raw_xmax: c_float,
    raw_ymin: c_float,
    raw_ymax: c_float,
) -> c_int {
    let result = std::panic::catch_unwind(|| {
        let file = PathBuf::from(
            unsafe { CStr::from_ptr(raw_path) }
                .to_str()
                .unwrap()
                .clone(),
        );
        let name = unsafe { CStr::from_ptr(raw_name) }
            .to_str()
            .unwrap()
            .to_owned();

        PlotDynoSetting::new(file, name, raw_xmin, raw_xmax, raw_ymin, raw_ymax).make_current();
    });
    match result {
        Ok(_) => 0,
        Err(_) => {
            PlotDynoSetting::current().err_callback("PLOTERROR: ERROR DYNOPLOT_init_config");
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn DYNOPLOT_SaveMultiPlot(
    raw_x: *const c_float,
    raw_speed_y: *const c_float,
    raw_rpm_y: *const c_float,
    raw_torsi_y: *const c_float,
    raw_hp_y: *const c_float,
    raw_odo_y: *const c_float,
    num_size: c_int,
) -> c_int {
    let result = std::panic::catch_unwind(|| {
        let size_n = num_size as usize;
        let slice_x: &[f32] = unsafe { std::slice::from_raw_parts(raw_x, size_n) };
        let slice_y: [Vec<f32>; 5] = unsafe {
            [
                std::slice::from_raw_parts(raw_speed_y, size_n).to_vec(),
                std::slice::from_raw_parts(raw_rpm_y, size_n).to_vec(),
                std::slice::from_raw_parts(raw_torsi_y, size_n).to_vec(),
                std::slice::from_raw_parts(raw_hp_y, size_n).to_vec(),
                std::slice::from_raw_parts(raw_odo_y, size_n).to_vec(),
            ]
        };

        rs_save_plot_dyno_all(slice_x, &slice_y, size_n);
    });
    match result {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

mod tests;
