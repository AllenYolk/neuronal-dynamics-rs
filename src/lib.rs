mod fn_neuron;
mod hh_neuron;
mod ml_neuron;
mod utils;
mod plot;

use std::{path::Path, fs::write};
use ml_neuron::*;
use clap::Subcommand;
use ode_solvers::rk4::*;
use utils::*;

#[derive(Subcommand)]
#[derive(Debug, Clone, Copy)]
#[command(rename_all = "PascalCase")]
pub enum NeuronType {
    HodgkinHuxley,
    FitzHughNagumo,
    MorrisLecar(MLNeuronSubcommand),
}

pub fn run(neuron_type: NeuronType, result_folder: Option<String>) -> Result<(), String>{
    match neuron_type {
        NeuronType::HodgkinHuxley => {
            println!("Running Hodgkin-Huxley model...");
            Ok(())
        }
        NeuronType::FitzHughNagumo => {
            println!("Running FitzHugh-Nagumo model...");
            Ok(())
        }
        NeuronType::MorrisLecar(MLNeuronSubcommand {subtype, i_inj, v_init, n_init, t_max}) => {
            println!("Running Morris-Lecar model...");
            let neuron = MLNeuron::from_subtype(subtype, i_inj);
            let mut msg = format!("subtype={:?}, i_inj={}, v_init={}, n_init={}, t_max={}", subtype, i_inj, v_init, n_init, t_max);
            append_line(&mut msg, &format!("{:#?}", neuron));
            println!("{}", msg);

            let init_state = MLNeuronState::new(v_init, n_init);
            let mut stepper = Rk4::new(neuron, 0.0, init_state, t_max, 0.02);
            let res = stepper.integrate();

            let Ok(stats) = res else {
                return Err(format!("Error: integration failed!"));
            };
            println!("{}", stats);

            if let Some(folder) = result_folder {
                // information
                let path = Path::new(&folder).join("info.txt");
                write(&path, msg).map_err(|x| x.to_string())?;

                // membrane potential
                let path = Path::new(&folder).join("membrane_potential.png");
                let y_out = stepper.y_out();
                let v: Vec<f64> = y_out.iter().map(|y| y[0]).collect();
                let t = stepper.x_out();
                plot::plot_membrane_potential(&path, &v, t).map_err(|x| x.to_string())?;

                // phase plane trajectory
                let path = Path::new(&folder).join("phase_plane.png");
                let n: Vec<f64> = y_out.iter().map(|y| y[1]).collect();
                plot::plot_phase_plane_trajectory(&path, &v, &n, "v", "n").map_err(|x| x.to_string())?;
            }

            Ok(())
        }
    }
}
