# neuronal-dynamics-rs

Simulate and visualize the dynamics of non-spiking neurons with Rust!

## Implementation

The simulations and visualizations are accomplished using the following crates:

* Solving ODEs: [`ode_solvers`](https://crates.io/crates/ode_solvers)
* CLI: [`clap`](https://crates.io/crates/clap)
* Visualization: [`plotters`](https://crates.io/crates/plotters)

## Contents

This crate currently supports the simulation of the following models:

* **Morris-Lecar Model**
    * Hopf bifurcation, saddle point limit cycle, saddle point bifurcation
* ......

Furthermore, the following visualization methods are provided:

* **Membrane potential -- time**
* **Phase plane trajectory**
