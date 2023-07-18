mod fn_neuron;
mod hh_neuron;
mod ml_neuron;
mod utils;

use ml_neuron::*;
use clap::Subcommand;
use ode_solvers::rk4::*;

#[derive(Subcommand)]
#[derive(Debug, Clone, Copy)]
#[command(rename_all = "PascalCase")]
pub enum NeuronType {
    HodgkinHuxley,
    FitzHughNagumo,
    MorrisLecar(MLNeuronSubcommand),
}

pub fn run(neuron_type: NeuronType) {
    match neuron_type {
        NeuronType::HodgkinHuxley => {
            println!("Running Hodgkin-Huxley model...");
        }
        NeuronType::FitzHughNagumo => {
            println!("Running FitzHugh-Nagumo model...");
        }
        NeuronType::MorrisLecar(MLNeuronSubcommand {subtype, i_inj, v_init, n_init, t_max}) => {
            println!("Running Morris-Lecar model...");
            let neuron = MLNeuron::from_subtype(subtype, i_inj);
            let init_state = MLNeuronState::new(v_init, n_init);
            let mut stepper = Rk4::new(neuron, 0.0, init_state, t_max, 0.1);
            let res = stepper.integrate();

            match res {
                Ok(stats) => {
                    println!("{}", stats);
                    println!("{:?}", stepper.x_out());
                    println!("{:?}", stepper.y_out());
                }
                Err(_) => {
                    eprintln!("Integration failed!");
                }
            }
            
        }
    }
}
