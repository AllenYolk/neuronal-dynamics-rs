mod hh_neuron;
mod fn_neuron;
mod ml_neuron;

use clap::ValueEnum;

#[derive(ValueEnum)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[value(rename_all = "PascalCase")]
pub enum NeuronType {
    HodgkinHuxley,
    FitzHughNagumo,
    MorrisLecar,
}

pub fn run(neuron_type: NeuronType) {
    match neuron_type {
        NeuronType::HodgkinHuxley => {
            println!("Running Hodgkin-Huxley model...");
        },
        NeuronType::FitzHughNagumo => {
            println!("Running FitzHugh-Nagumo model...");
        },
        NeuronType::MorrisLecar => {
            println!("Running Morris-Lecar model...");
        },
    }
}
