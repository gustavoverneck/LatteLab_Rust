// src/main.rs

#![allow(unused)]

// Imports
mod core;
mod utils;
use core::lbm::LBM;

fn main() {
    // Initialize LBM simulation
    let mut lbm = LBM::new(128, 128, 1, "D2Q9".to_string(), 0.1);
    // Set initial conditions
    lbm.set_conditions(|lbm, x, y, z, n| {
        if x == 0 {
            lbm.u[n].x = 0.0f32;
            lbm.u[n].y = 0.1f32;
            lbm.u[n].z = 0.0f32;
            lbm.density[n] = 1.2f32;
        } else {
            lbm.u[n].x = 0.0f32;
            lbm.u[n].y = 0.0f32;
            lbm.u[n].z = 0.0f32;
            lbm.density[n] = 1.1f32;
        }
    });

    lbm.run(1000);
    lbm.output_to("output.csv").expect("Failed to write output file.");
}
