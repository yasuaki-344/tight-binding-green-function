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

#[derive(Serialize, Deserialize)]
pub struct ScalingParameters {
    /// hopping energy in no-strain.
    pub h0: f64,
    /// 1st scaling parameter.
    pub n: f64,
    /// 2nd scaling parameter.
    pub nc: f64,
    /// 3rd scaling parameter.
    pub rc: f64,
}

/// Tight-Binding interaction parameters.
#[derive(Serialize, Deserialize)]
pub struct InteractionParameters {
    /// (ss sigma) element.
    pub s_s_sigma: ScalingParameters,
    /// (ps sigma) element.
    pub p_s_sigma: ScalingParameters,
    /// (pp sigma) element.
    pub p_p_sigma: ScalingParameters,
    /// (pp pi) element.
    pub p_p_pi: ScalingParameters,
    /// (s^*s sigma) element.
    pub s_ast_s_sigma: ScalingParameters,
    /// (s^*p sigma) element.
    pub s_ast_p_sigma: ScalingParameters,
    /// (s^*s^* sigma) element.
    pub s_ast_s_ast_sigma: ScalingParameters,
    /// (ds sigma) element.
    pub d_s_sigma: ScalingParameters,
    /// (ds^* sigma) element.
    pub d_s_ast_sigma: ScalingParameters,
    /// (dp sigma) element.
    pub d_p_sigma: ScalingParameters,
    /// (dp pi) element.
    pub d_p_pi: ScalingParameters,
    /// (dd sigma) element.
    pub d_d_sigma: ScalingParameters,
    /// (dd pi) element.
    pub d_d_pi: ScalingParameters,
    /// (dd delta) element.
    pub d_d_delta: ScalingParameters,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
    /// distance between nearest neighboring atoms in bulk crystal [Ang].
    atomic_distance: f64,
    /// Tight-Binding Onsite Parameter of anion.
    onsite_anion: OnsiteParameters,
    /// Tight-Binding Onsite Parameter of cation.
    onsite_cation: OnsiteParameters,
    /// Tight-Binding Overlap Parameter of the transition from anion.
    overlap_from_anion: InteractionParameters,
    /// Tight-Binding Overlap Parameter of the transition from cation.
    overlap_from_cation: InteractionParameters,
}
