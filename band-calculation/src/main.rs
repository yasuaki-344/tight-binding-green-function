extern crate ndarray;
extern crate ndarray_linalg;
mod atom;
mod tight_binding;
use ndarray::*;
use ndarray_linalg::*;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "band-structure.png";

fn plot_data(x: Vec<f32>, y1: Vec<f32>, y2: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let image_width = 1080;
    let image_height = 720;

    let root_area =
        BitMapBackend::new(OUT_FILE_NAME, (image_width, image_height)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("band structure", ("sans-serif", 30))
        .build_cartesian_2d(0f32..std::f32::consts::PI, -4.2f32..0.2f32)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    // plot data
    cc.draw_series(LineSeries::new(
        x.iter().zip(y1.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?;
    cc.draw_series(LineSeries::new(
        x.iter().zip(y2.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?;

    // save plot data as image file
    root_area.present().expect("Unable to write result to file");
    Ok(())
}

fn main() {
    let wave_number_point = 100;
    let mut x = Vec::new();
    let mut y1 = Vec::new();
    let mut y2 = Vec::new();
    for ik in 0..wave_number_point {
        let wave_number = (ik as f32) * std::f32::consts::PI / (wave_number_point as f32 - 1.0);
        println!("wave number: {}", wave_number);
        x.push(wave_number);
        // TODO: create Hamiltonian matrix
        let hamiltonian = arr2(&[
            [c32::new(-2.0, 0.0), 1.0 + c32::new(0., wave_number).exp()],
            [1.0 + c32::new(0., -wave_number).exp(), c32::new(-2.0, 0.0)],
        ]);
        let (eigen, _) = hamiltonian.clone().eigh(UPLO::Upper).unwrap();
        y1.push(eigen[0]);
        y2.push(eigen[1]);
    }
    // TODO: output eigen values to file
    let _ = plot_data(x, y1, y2);
}
