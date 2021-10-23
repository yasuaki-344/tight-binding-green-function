extern crate ndarray;
extern crate ndarray_linalg;
mod atom;
mod tight_binding;
use ndarray::*;
use ndarray_linalg::*;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "band-structure.png";

fn plot_data() -> Result<(), Box<dyn std::error::Error>> {
    let image_width = 1080;
    let image_height = 720;

    let root_area =
        BitMapBackend::new(OUT_FILE_NAME, (image_width, image_height)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let x_axis = (0f32..std::f32::consts::PI).step(0.01 * std::f32::consts::PI);

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
    let _ = cc.draw_series(LineSeries::new(
        x_axis
            .values()
            .map(|x| (x, -2.0 + (2.0 + 2.0 * x.cos()).sqrt())),
        &RED,
    ));
    let _ = cc.draw_series(LineSeries::new(
        x_axis
            .values()
            .map(|x| (x, -2.0 - (2.0 + 2.0 * x.cos()).sqrt())),
        &RED,
    ));

    // save plot data as image file
    root_area.present().expect("Unable to write result to file");
    Ok(())
}

fn main() {
    let wave_number_point = 100;
    for ik in 0..wave_number_point {
        let wave_number = (ik as f64) * (1.0 / std::f64::consts::PI);
        println!("wave number: {}", wave_number);
        // TODO: create Hamiltonian matrix
        let hamiltonian = arr2(&[
            [c64::new(-2.0, 0.0), 1.0 + c64::new(0., wave_number).exp()],
            [1.0 + c64::new(0., -wave_number).exp(), c64::new(-2.0, 0.0)],
        ]);
        let (eigen, _) = hamiltonian.clone().eigh(UPLO::Upper).unwrap();
        println!("eigenvalues = \n{:?}", eigen);
    }
    // TODO: output eigen values to file
    let _ = plot_data();
}
