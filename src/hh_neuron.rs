use clap::{Args, ValueEnum};
use ode_solvers::*;
use crate::utils::get_ion_current;

#[derive(Args)]
#[derive(Debug, Clone, Copy)]
pub struct HHNeuronSubcommand {
    pub subtype: HHNeuronSubtype,

    #[arg(default_value_t = 0.0)]
    #[arg(short='I', long)]
    pub i_inj: f64,

    #[arg(default_value_t = 6.3)]
    #[arg(short='T', long)]
    pub t_real: f64,

    #[arg(default_value_t = -65.0)]
    #[arg(short='V', long)]
    pub v_init: f64,

    #[arg(default_value_t = 0.32)]
    #[arg(short='N', long)]
    pub n_init: f64,

    #[arg(default_value_t = 0.06)]
    #[arg(short='M', long)]
    pub m_init: f64,

    #[arg(default_value_t = 0.6)]
    #[arg(short='H', long)]
    pub h_init: f64,

    #[arg(default_value_t = 500.0)]
    #[arg(short, long)]
    pub t_max: f64,
}

#[derive(ValueEnum)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[value(rename_all = "PascalCase")]
pub enum HHNeuronSubtype {
    Standard,
    ReducedTraubMiles,
    WangBuzsaki,
}

pub type HHNeuronState = SVector<f64, 4>;
type Time = f64;

#[derive(Debug, Clone, Copy)]
pub struct HHNeuron {
    c_m: f64,
    g_na: f64,
    e_na: f64,
    g_k: f64,
    e_k: f64,
    g_l: f64,
    e_l: f64,
    alpha_n_1: f64,
    alpha_n_2: f64,
    alpha_n_3: f64,
    alpha_n_4: f64,
    beta_n_1: f64,
    beta_n_2: f64,
    beta_n_3: f64,
    alpha_m_1: f64,
    alpha_m_2: f64,
    alpha_m_3: f64,
    alpha_m_4: f64,
    beta_m_1: f64,
    beta_m_2: f64,
    beta_m_3: f64,
    alpha_h_1: f64,
    alpha_h_2: f64,
    alpha_h_3: f64,
    beta_h_1: f64,
    beta_h_2: f64,
    beta_h_3: f64,
    q: f64,
    t_base: f64,
    t_real: f64,
    i_inj: f64,
}

impl HHNeuron {
    pub fn new(
        c_m: f64,
        g_na: f64,
        e_na: f64,
        g_k: f64,
        e_k: f64,
        g_l: f64,
        e_l: f64,
        alpha_n_1: f64,
        alpha_n_2: f64,
        alpha_n_3: f64,
        alpha_n_4: f64,
        beta_n_1: f64,
        beta_n_2: f64,
        beta_n_3: f64,
        alpha_m_1: f64,
        alpha_m_2: f64,
        alpha_m_3: f64,
        alpha_m_4: f64,
        beta_m_1: f64,
        beta_m_2: f64,
        beta_m_3: f64,
        alpha_h_1: f64,
        alpha_h_2: f64,
        alpha_h_3: f64,
        beta_h_1: f64,
        beta_h_2: f64,
        beta_h_3: f64,
        q: f64,
        t_base: f64, // degrees Celsius
        t_real: f64, // degrees Celsius
        i_inj: f64,
    ) -> Self {
        Self {
            c_m,
            g_na,
            e_na,
            g_k,
            e_k,
            g_l,
            e_l,
            alpha_n_1,
            alpha_n_2,
            alpha_n_3,
            alpha_n_4,
            beta_n_1,
            beta_n_2,
            beta_n_3,
            alpha_m_1,
            alpha_m_2,
            alpha_m_3,
            alpha_m_4,
            beta_m_1,
            beta_m_2,
            beta_m_3,
            alpha_h_1,
            alpha_h_2,
            alpha_h_3,
            beta_h_1,
            beta_h_2,
            beta_h_3,
            q,
            t_base,
            t_real,
            i_inj,
        }
    }

    pub fn from_subtype(subtype: HHNeuronSubtype, i_inj: f64, t_real: f64) -> Self {
        match subtype {
            HHNeuronSubtype::Standard => Self::new(
                1., 120., 50., 36., -77., 0.3, -54.4, 
                0.01, 55., 55., 10., 
                0.125, 65., 80., 
                0.1, 40., 40., 10.,
                4., 65., 18.,
                0.07, 65., 20.,
                1., 35., 10.,
                3., 6.3, t_real, i_inj
            ),
            HHNeuronSubtype::ReducedTraubMiles => Self::new(
                1., 100., 50., 80., -100., 0.1, -67.,
                0.032, 52., 52., 5.,
                0.5, 57., 40.,
                0.32, 54., 54., 4.,
                7.698, 58.929, 26.260,
                0.128, 50., 18.,
                4., 27., 5.,
                3., 6.3, t_real, i_inj
            ),
            HHNeuronSubtype::WangBuzsaki => Self::new(
                1., 35., 55., 9., -90., 0.1, -65.,
                0.05, 34., 34., 10.,
                0.625, 44., 80.,
                0.1, 35., 35., 10.,
                4., 60., 18.,
                0.35, 58., 20., 
                5., 28., 10.,
                3., 6.3, t_real, i_inj
            ),
        }
    }
}

impl ode_solvers::System<HHNeuronState> for HHNeuron {
    fn system(&self, _t: Time, y: &HHNeuronState, dydt: &mut HHNeuronState) {
        let v = y[0];
        let n = y[1].clamp(0., 1.);
        let m = y[2].clamp(0., 1.);
        let h = y[3].clamp(0., 1.);

        let alpha_n = self.alpha_n_1 * (v + self.alpha_n_2) / (1. - (-(v + self.alpha_n_3) / self.alpha_n_4).exp());
        let beta_n = self.beta_n_1 * (-(v + self.beta_n_2) / self.beta_n_3).exp();
        let alpha_m = self.alpha_m_1 * (v + self.alpha_m_2) / (1. - (-(v + self.alpha_m_3) / self.alpha_m_4).exp());
        let beta_m = self.beta_m_1 * (-(v + self.beta_m_2) / self.beta_m_3).exp();
        let alpha_h = self.alpha_h_1 * (-(v + self.alpha_h_2) / self.alpha_h_3).exp();
        let beta_h = self.beta_h_1 / (1. + (-(v + self.beta_h_2) / self.beta_h_3).exp());
        let phi = self.q.powf((self.t_real - self.t_base) / 10.);

        dydt[0] = (self.i_inj 
            - get_ion_current(v, self.g_na * m.powi(3) * h, self.e_na)
            - get_ion_current(v, self.g_k * n.powi(4), self.e_k)
            - get_ion_current(v, self.g_l, self.e_l))
            / self.c_m;
        dydt[1] = phi * (alpha_n * (1. - n) - beta_n * n);
        dydt[2] = phi * (alpha_m * (1. - m) - beta_m * m);
        dydt[3] = phi * (alpha_h * (1. - h) - beta_h * h);
    }
}
