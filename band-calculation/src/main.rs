mod tight_binding;

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
}
