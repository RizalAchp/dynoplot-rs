#![allow(dead_code)]
#![allow(unused_variables)]
#![macro_use]
use plotters::coord::Shift;
use plotters::prelude::*;
use std::ffi::CStr;
use std::ops::Range;
use std::os::raw::{c_char, c_int, c_void};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

const COLOR_BLUE: RGBColor = RGBColor(17, 16, 92);
const COLOR_PURPLE: RGBColor = RGBColor(85, 7, 102);
const COLOR_MAGENTA: RGBColor = RGBColor(135, 0, 102);
const COLOR_PINK: RGBColor = RGBColor(179, 0, 95);
const COLOR_RED: RGBColor = RGBColor(215, 41, 81);
const COLOR_ORANGE: RGBColor = RGBColor(240, 81, 63);
const COLOR_ORANGE_YELLOW: RGBColor = RGBColor(253, 123, 40);
const COLOR_YELLOW: RGBColor = RGBColor(255, 166, 0);

const COLORS: [RGBColor; 8] = [
    COLOR_BLUE,
    COLOR_PURPLE,
    COLOR_MAGENTA,
    COLOR_PINK,
    COLOR_RED,
    COLOR_ORANGE,
    COLOR_ORANGE_YELLOW,
    COLOR_YELLOW,
];

#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct PlotDynoSetting {
    pub file: PathBuf,
    pub name: String,
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub callback: Option<unsafe fn(*const c_char, c_int) -> c_void>,
}

impl PlotDynoSetting {
    pub fn new(file: PathBuf, name: String, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Self {
        Self {
            file,
            name,
            xmin,
            xmax,
            ymin,
            ymax,
            callback: None,
        }
    }
    pub fn set_callback(mut self, callback: unsafe fn(*const c_char, c_int) -> c_void) -> Self {
        self.callback = Some(callback);
        self
    }
    pub fn current() -> Arc<PlotDynoSetting> {
        CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn err_callback(&self, msg: &str) {
        if let Some(callback) = &self.callback {
            let cstr = CStr::from_bytes_with_nul(msg.as_bytes()).unwrap_or_default();
            unsafe {
                callback(cstr.as_ptr(), cstr.to_bytes().len() as i32);
            }
        }
    }
}

thread_local! {
    pub static CONFIG: RwLock<Arc<PlotDynoSetting>> = RwLock::new(Default::default());
}

pub fn build_plot<X, Y>(
    root: &DrawingArea<SVGBackend, Shift>,
    name: String,
    xrange: Range<f32>,
    yrange: Range<f32>,
    x: X,
    y: Y,
    color: &RGBColor,
) -> ()
where
    X: IntoIterator<Item = f32>,
    Y: IntoIterator<Item = f32>,
{
    let mut chart = match ChartBuilder::on(&root)
        .margin(i32::from(10))
        .set_all_label_area_size(i32::from(60))
        .build_cartesian_2d(xrange, yrange)
    {
        Ok(c) => c,
        Err(e) => {
            PlotDynoSetting::current().err_callback(&format!(
                "PLOTERROR: Error on building cartesian 2d => {}",
                e.to_string()
            ));
            panic!("{}", e);
        }
    };

    match chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(20)
        .x_label_formatter(&|v| format!("{:.2}", v))
        .y_label_formatter(&|v| format!("{:.2}", v))
        .label_style(("sans-serif", 18, &BLACK))
        .draw()
    {
        Ok(k) => k,
        Err(e) => {
            PlotDynoSetting::current().err_callback(&format!(
                "PLOTERROR: Error on draw mesh => {}",
                e.to_string()
            ));
            panic!("{}", e);
        }
    };

    match chart.draw_series(LineSeries::new(y.into_iter().zip(x.into_iter()), &color)) {
        Ok(k) => k,
        Err(e) => {
            PlotDynoSetting::current().err_callback(&format!(
                "PLOTERROR: Error on draw series => {}",
                e.to_string()
            ));
            panic!("{}", e);
        }
    }
    .label(&name)
    .legend(|d| PathElement::new(vec![d, (d.0 + 20, d.1)], color.filled()));

    match chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(color.mix(0.8))
        .draw()
    {
        Ok(k) => k,
        Err(e) => {
            PlotDynoSetting::current().err_callback(&format!(
                "PLOTERROR: Error on configure series labels => {}",
                e.to_string()
            ));
            panic!("{}", e);
        }
    }
}

pub fn rs_save_plot_dyno(setting: &PlotDynoSetting, x: Vec<f32>, y: Vec<f32>, size: usize) {
    let root_area = SVGBackend::new(&setting.file, (1280, 720)).into_drawing_area();
    root_area.margin(30, 30, 30, 30);
    root_area.fill(&WHITE).unwrap();
    root_area.titled(&setting.name, ("sans-serif", 48)).unwrap();

    build_plot(
        &root_area,
        setting.name.clone(),
        setting.xmin..setting.xmax,
        setting.ymin..setting.ymax,
        x,
        y,
        &COLOR_BLUE,
    );

    match root_area.present() {
        Ok(k) => k,
        Err(e) => {
            setting.err_callback(&format!(
                "PLOTERROR: Error on Saving plot image => {}",
                e.to_string()
            ));
            panic!();
        }
    }
}

pub fn rs_save_plot_dyno_all(x: &[f32], y: &[Vec<f32>; 5], size: usize) {
    let setting = PlotDynoSetting::current();
    let root_area = SVGBackend::new(&setting.file, (1280, 720)).into_drawing_area();
    root_area.margin(30, 30, 30, 30);
    root_area.fill(&WHITE).unwrap();
    root_area.titled(&setting.name, ("sans-serif", 48)).unwrap();

    for (i, yitem) in y.into_iter().enumerate() {
        build_plot(
            &root_area,
            format!("{}-{}", setting.name.clone(), i),
            setting.xmin..setting.xmax,
            setting.ymin..setting.ymax,
            x.to_vec(),
            yitem.to_owned(),
            &COLORS[i],
        );
    }

    match root_area.present() {
        Ok(k) => k,
        Err(e) => {
            setting.err_callback(&format!(
                "PLOTERROR: on saving plot image => {}",
                e.to_string()
            ));
            panic!("{}", e);
        }
    }
}
