extern crate blas;
extern crate nalgebra as na;
extern crate openblas_src;
mod atom;
mod tight_binding;
use nalgebra_lapack::SymmetricEigen;
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
    // TODO: create tight-binding parameters
    let tb_parameter = tight_binding::OnsiteParameters {
        s: -2.0196,
        p: 4.5448,
        s_ast: 19.6748,
        d: 14.1836,
    };
    let json = serde_json::to_string(&tb_parameter).unwrap();
    println!("{}", json);
    // TODO: create unit cell object
    // TODO: generate output file
    let wave_number_point = 100;
    for ik in 0..wave_number_point {
        println!("wave number: {}", ik);
        // TODO: create Hamiltonian matrix
        // TODO: solve eigen value problem of Hamiltonian matrix
        // TODO: output eigen values to file
    }
    let _ = plot_data();

    let matrix = na::Matrix2::new(1.0, 0.0, 0.0, 3.0);
    println!("{}", matrix);
    let eigen = SymmetricEigen::new(matrix);
    let eigen_values = eigen.eigenvalues;
    println!("{}", eigen_values);
}
