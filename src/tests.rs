#![cfg(test)]
use std::{ops::Neg, path::PathBuf};

use crate::{dynoplot::rs_save_plot_dyno_all, rs_save_plot_dyno};

#[test]
fn test_get_config() {
    crate::dynoplot::PlotDynoSetting {
        file: PathBuf::from("plot.svg"),
        name: "test_plot".to_owned(),
        xmin: 1f32.neg(),
        xmax: 1f32,
        ymin: 1f32.neg(),
        ymax: 1f32,
    }
    .make_current();
    let cmp = crate::dynoplot::PlotDynoSetting {
        file: PathBuf::from("plot.svg"),
        name: "test_plot".to_owned(),
        xmin: 1f32.neg(),
        xmax: 1f32,
        ymin: 1f32.neg(),
        ymax: 1f32,
    };
    let setting = crate::dynoplot::PlotDynoSetting::current();
    assert_eq!(
        setting.as_ref(),
        &cmp,
        "comparing setting {setting:?} == {cmp:?}"
    );
}
#[test]
fn test_gen_plot() {
    let result = std::panic::catch_unwind(|| {
        let setting = crate::dynoplot::PlotDynoSetting {
            file: PathBuf::from("plot.svg"),
            name: "test_plot".to_owned(),
            xmin: 1f32.neg(),
            xmax: 1f32,
            ymin: 1f32.neg(),
            ymax: 1f32,
        };
        dbg!(&setting);
        let data = (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x));
        let x_item: Vec<f32> = data.clone().map(|(x, _)| x).collect();
        let y_item: Vec<f32> = data.clone().map(|(_, y)| y).collect();
        let len = y_item.len();
        rs_save_plot_dyno(&setting, x_item, y_item, len);
    });

    assert_eq!(
        result.is_ok(),
        true,
        "test and check result {result:?} is not error"
    );
}

#[test]
fn test_gen_multi_plot() {
    let result = std::panic::catch_unwind(|| {
        crate::dynoplot::PlotDynoSetting {
            file: PathBuf::from("plot.svg"),
            name: "test_plot".to_owned(),
            xmin: 1f32.neg(),
            xmax: 1f32,
            ymin: 1f32.neg(),
            ymax: 1f32,
        }
        .make_current();
        let data = (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x));
        let x_item: Vec<f32> = data.clone().map(|(x, _)| x).collect();
        let y_item: [Vec<f32>; 5] = [
            (-50..=50)
                .map(|x| (x as f32 / 50.0) + 69f32.sin())
                .collect::<Vec<_>>(),
            (-50..=50)
                .map(|x| (x as f32 / 50.0) + 29f32.sin())
                .collect::<Vec<_>>(),
            (-50..=50)
                .map(|x| (x as f32 / 50.0) + 39f32.sin())
                .collect::<Vec<_>>(),
            (-50..=50)
                .map(|x| (x as f32 / 50.0) + 89f32.sin())
                .collect::<Vec<_>>(),
            (-50..=50)
                .map(|x| (x as f32 / 50.0) + 59f32.sin())
                .collect::<Vec<_>>(),
        ];
        rs_save_plot_dyno_all(&x_item, &y_item, x_item.len());
    });

    assert_eq!(
        result.is_ok(),
        true,
        "test and check result {result:?} is not error"
    );
}
