use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TightBindingOnsiteParameters {
    /// s-orbit onsite energy parameter.
    s: f64,
    /// p-orbit onsite energy parameter.
    p: f64,
    /// s*-orbit onsite energy parameter.
    s_ast: f64,
    /// d-orbit onsite energy parameter.
    d: f64,
}

#[derive(Serialize, Deserialize)]
struct TB_ScalingParameters {
    /// hopping energy in no-strain.
    h0: f64,
    /// 1st scaling parameter.
    n: f64,
    /// 2nd scaling parameter.
    nc: f64,
    /// 3rd scaling parameter.
    rc: f64,
}

/// Tight-Binding interaction parameters.
#[derive(Serialize, Deserialize)]
struct TB_InteractionParameters {
    /// (ss sigma) element.
    s_s_sigma: TB_ScalingParameters,
    /// (ps sigma) element.
    p_s_sigma: TB_ScalingParameters,
    /// (pp sigma) element.
    p_p_sigma: TB_ScalingParameters,
    /// (pp pi) element.
    p_p_pi: TB_ScalingParameters,
    /// (s^*s sigma) element.
    s_ast_s_sigma: TB_ScalingParameters,
    /// (s^*p sigma) element.
    s_ast_p_sigma: TB_ScalingParameters,
    /// (s^*s^* sigma) element.
    s_ast_s_ast_sigma: TB_ScalingParameters,
    /// (ds sigma) element.
    d_s_sigma: TB_ScalingParameters,
    /// (ds^* sigma) element.
    d_s_ast_sigma: TB_ScalingParameters,
    /// (dp sigma) element.
    d_p_sigma: TB_ScalingParameters,
    /// (dp pi) element.
    d_p_pi: TB_ScalingParameters,
    /// (dd sigma) element.
    d_d_sigma: TB_ScalingParameters,
    /// (dd pi) element.
    d_d_pi: TB_ScalingParameters,
    /// (dd delta) element.
    d_d_delta: TB_ScalingParameters,
}

#[derive(Serialize, Deserialize)]
struct TightBindingParameter {
    /// distance between nearest neighboring atoms in bulk crystal [Ang].
    atomic_distance: f64,
    /// Tight-Binding Onsite Parameter of anion.
    onsite_anion: TightBindingOnsiteParameters,
    /// Tight-Binding Onsite Parameter of cation.
    onsite_cation: TightBindingOnsiteParameters,
    /// Tight-Binding Overlap Parameter of the transition from anion.
    overlap_from_anion: TB_InteractionParameters,
    /// Tight-Binding Overlap Parameter of the transition from cation.
    overlap_from_cation: TB_InteractionParameters,
}

mod tight_binding {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct OnsiteParameters {
        /// s-orbit onsite energy parameter.
        pub s: f64,
        /// p-orbit onsite energy parameter.
        pub p: f64,
        /// s*-orbit onsite energy parameter.
        pub s_ast: f64,
        /// d-orbit onsite energy parameter.
        pub d: f64,
    }
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
