#![allow(dead_code)]

use crate::utils::*;
use ode_solvers::*;
use clap::{ValueEnum, Args};

#[derive(Args)]
#[derive(Debug, Clone, Copy)]
pub struct MLNeuronSubcommand {
    pub subtype: MLNeuronSubtype,

    #[arg(default_value_t = 0.0)]
    #[arg(short, long)]
    pub i_inj: f64,

    #[arg(default_value_t = -60.0)]
    #[arg(short, long)]
    pub v_init: f64,

    #[arg(default_value_t = 0.0)]
    #[arg(short, long)]
    pub n_init: f64,

    #[arg(default_value_t = 100.0)]
    #[arg(short, long)]
    pub t_max: f64,
}

#[derive(ValueEnum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[value(rename_all = "PascalCase")]
pub enum MLNeuronSubtype {
    Hopf,
    SaddleNodeLimitCycle,
    SaddleNodeBifurcation,
}

pub type MLNeuronState = SVector<f64, 2>;
type Time = f64;

#[derive(Debug, Clone, Copy)]
pub struct MLNeuron {
    c_m: f64,
    g_l: f64,
    e_l: f64,
    g_k: f64,
    e_k: f64,
    g_ca: f64,
    e_ca: f64,
    phi: f64,
    v1: f64,
    v2: f64,
    v3: f64,
    v4: f64,
    i_inj: f64,
}

impl MLNeuron {
    pub fn new(
        c_m: f64,
        g_l: f64,
        e_l: f64,
        g_k: f64,
        e_k: f64,
        g_ca: f64,
        e_ca: f64,
        phi: f64,
        v1: f64,
        v2: f64,
        v3: f64,
        v4: f64,
        i_inj: f64,
    ) -> Self {
        Self {
            c_m,
            g_l,
            e_l,
            g_k,
            e_k,
            g_ca,
            e_ca,
            phi,
            v1,
            v2,
            v3,
            v4,
            i_inj,
        }
    }

    pub fn from_subtype(subtype: MLNeuronSubtype, i_inj: f64) -> Self {
        match subtype {
            MLNeuronSubtype::Hopf => Self::new(
                20.0, 2.0, -60.0, 8.0, -84.0, 4.4, 120.0, 0.04, -1.2, 18.0, 2.0, 30.0, i_inj,
            ),
            MLNeuronSubtype::SaddleNodeLimitCycle => Self::new(
                20.0, 2.0, -60.0, 8.0, -84.0, 4.0, 120.0, 0.067, -1.2, 18.0, 12.0, 17.4, i_inj,
            ),
            MLNeuronSubtype::SaddleNodeBifurcation => Self::new(
                20.0, 2.0, -60.0, 8.0, -84.0, 4.0, 120.0, 0.23, -1.2, 18.0, 12.0, 17.4, i_inj,
            ),
        }
    }
}

impl ode_solvers::System<MLNeuronState> for MLNeuron {
    fn system(&self, _t: Time, y: &MLNeuronState, dydt: &mut MLNeuronState) {
        let m = 0.5 * (1.0 + ((y[0] - self.v1) / self.v2).tanh());
        let tau_n = 1.0 / ((y[0] - self.v3) / (2.0 * self.v4)).cosh();
        let n = 0.5 * (1.0 + ((y[0] - self.v3) / self.v4).tanh());

        dydt[0] = (self.i_inj
            - get_ion_current(y[0], self.g_l, self.e_l)
            - get_ion_current(y[0], self.g_k * y[1], self.e_k)
            - get_ion_current(y[0], self.g_ca * m, self.e_ca))
            / self.c_m;
        dydt[1] = self.phi * (n - y[1]) / tau_n;
    }
}
