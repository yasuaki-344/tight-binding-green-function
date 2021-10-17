extern crate blas;
extern crate nalgebra as na;
extern crate openblas_src;
mod atom;
mod tight_binding;
use nalgebra_lapack::SymmetricEigen;
use plotters::prelude::*;

fn plot_data() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![(1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0), (5.0, 25.0)];
    let xs: Vec<f64> = data.iter().map(|(x, _)| *x).collect();
    let ys: Vec<f64> = data.iter().map(|(_, y)| *y).collect();
    let image_width = 1080;
    let image_height = 720;
    let root =
        BitMapBackend::new("band-structure.png", (image_width, image_height)).into_drawing_area();
    root.fill(&WHITE)?;
    let (y_min, y_max) = ys
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));
    let caption = "Band Structure";
    let font = ("sans-serif", 20);
    let mut chart = ChartBuilder::on(&root)
        .caption(caption, font.into_font())
        .margin(10)
        .x_label_area_size(16)
        .y_label_area_size(42)
        .build_cartesian_2d(*xs.first().unwrap()..*xs.last().unwrap(), y_min..y_max)?;

    chart.configure_mesh().draw()?;

    let point_series = xs
        .iter()
        .zip(ys.iter())
        .map(|(x, y)| Circle::new((*x, *y), 4, &RED));
    chart.draw_series(point_series)?;
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
